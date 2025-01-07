use std::fs;
use std::ffi::CString;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use serde_json::{json, Value};

pub fn call_python(file_path: &str, module: &str, class: &str, object_args: Option<Vec<&str>>, method: &str, args: Option<Vec<&str>>, kwargs: Option<Vec<(&str, &str)>>) -> PyResult<Value> {

    let file_name = format!("{}.py",module);
    let code = fs::read_to_string(file_path).expect("Python file not found");
    let module = module.to_string();
    
    Python::with_gil(|py| {
        let code_cstr = CString::new(code).unwrap();
        let file_name_cstr = CString::new(file_name).unwrap();
        let module_cstr = CString::new(module).unwrap();

        let py_module = PyModule::from_code(py, &code_cstr, &file_name_cstr, &module_cstr)?;

        let py_object = py_module.getattr(class)?;

        let py_class: Bound<'_, PyAny>;

        if let Some(object_args) = object_args{
            let method_args = PyTuple::new(py, object_args).unwrap();
            py_class = py_object.call1(method_args)?;
        }else{
            py_class = py_object.call0()?;
        }

        let raw_result = match (args, kwargs) {
            (None, None) => {
                py_class.call_method0(method)?
            }
            (Some(arg_list), None) => {
                let method_args = PyTuple::new(py, arg_list).unwrap();
                py_class.call_method1(method, method_args)?
            }
            (Some(arg_list), Some(kwarg_list)) => {
                let method_args = PyTuple::new(py, arg_list).unwrap();
                let method_kwargs = PyDict::new(py);
                for (key, value) in kwarg_list {
                    method_kwargs.set_item(key, value)?;
                }
                py_class.call_method(method, method_args, Some(&method_kwargs))?
            }
            (None, Some(kwarg_list)) => {
                let method_kwargs = PyDict::new(py);
                for (key, value) in kwarg_list {
                    method_kwargs.set_item(key, value)?;
                }
                py_class.call_method(method, (), Some(&method_kwargs))?
            }
        };

        convert_to_json(raw_result)

    })
}

fn convert_to_json(py_obj: pyo3::Bound<'_, pyo3::PyAny, >) -> PyResult<Value> {
    if let Ok(py_list) = py_obj.downcast::<PyList>() {
        let json_array: PyResult<Vec<Value>> = py_list
            .iter()
            .map(|item| convert_to_json(item))
            .collect();
        json_array.map(Value::Array)
    } else if let Ok(py_dict) = py_obj.downcast::<PyDict>() {
        let mut json_map = serde_json::Map::new();
        for (key, value) in py_dict {
            let key_str = key.extract::<String>()?;
            let json_value = convert_to_json(value)?;
            json_map.insert(key_str, json_value);
        }
        Ok(Value::Object(json_map))
    } else if let Ok(py_str) = py_obj.extract::<String>() {
        Ok(Value::String(py_str))
    } else {
        Ok(Value::String(format!("{:?}", py_obj)))
    }
}


pub fn handle_python_call(file_path: &str, module: &str, object: &str, object_args: Option<Vec<&str>>, method: &str, args: Option<Vec<&str>>, kwargs: Option<Vec<(&str, &str)>>) -> Result<String, String> {
    match call_python(file_path, module, object, object_args, method, args, kwargs) {
        Ok(output) => {
            let result = json!({
                "success": true,
                "output": output
            });
            Ok(result.to_string())
        }
        Err(err) => {
            let result = json!({
                "success": false,
                "error": err.to_string()
            });
            Err(result.to_string())
        }
    }
}