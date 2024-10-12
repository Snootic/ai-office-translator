import pandas as pd
import json, deepl

#todo add method overload

class Glossario():
    def __init__(self, api_key: str) -> None:
        self.API_KEY = api_key
        self.glossario: deepl.GlossaryInfo | str

    def create_from_excel(self, spreadsheet_path, file_name, file_path, source_language, target_language, excluded_keys:list = None) -> None:
        df = pd.read_excel(spreadsheet_path)

        if not excluded_keys:
            excluded_keys = []

        if df.shape[1] != 2:
            raise ValueError("O DataFrame deve ter exatamente duas colunas.")


        dicionario = {}
        for row in df.itertuples(index=False):
            chave, valor = row[0], row[1]
            try:
                chave = chave.rstrip()
                valor = valor.rstrip()
            except Exception as e:
                print(e)
                print(chave,valor)
                
            if (chave not in excluded_keys and 
                valor not in excluded_keys and
                pd.notna(chave) and 
                pd.notna(valor)):
                if chave not in dicionario:
                    dicionario[chave] = valor

        with open(file_path, 'w',encoding='utf-8') as file:
            json.dump(dicionario, file, indent=2,ensure_ascii=False)
            
        json_file = self.load_json(file_path)
        
        glossary = self.create_glossary(glossary_name=file_name,source_language=source_language, target_language=target_language, glossary=json_file)
        
        return glossary.__dict__
            

    def create_glossary(self,glossary_name:str,source_language:str,target_language:str,glossary:dict):
        translator = deepl.Translator(self.API_KEY)

        self.glossary = translator.create_glossary(
            glossary_name,
            source_lang=source_language,
            target_lang=target_language,
            entries=glossary
        )
        return self.glossary
    
    def load_json(self,json_file) -> dict:
        with open(json_file,encoding="utf-8") as json_dict:
            glossary = json.load(json_dict)

        return glossary
    
    def get_glossaries(self):
        translator = deepl.Translator(self.API_KEY)
        
        glossaries = translator.list_glossaries()
        
        result = []
        
        for glossary in glossaries:
            result.append(glossary.__dict__)

        return result

    def delete_glossary(self,glossary:deepl.GlossaryInfo|str):
        return deepl.Translator(self.API_KEY).delete_glossary(glossary=glossary)
    
    def get_glossary_entries(self,glossary:deepl.GlossaryInfo|str):
        return deepl.Translator(self.API_KEY).get_glossary_entries(glossary)
    
    def get_glossary_languages(self):
        glossary_languages = deepl.Translator(self.API_KEY).get_glossary_languages()
        
        languages = []
        
        for glossary_language in glossary_languages:
            languages.append(glossary_language.__dict__)
        
        return languages
    
    def create_glossary_from_csv(self,csv_file_path:str,glossary_name:str,source_language:str,target_language:str):
        translator = deepl.Translator(self.API_KEY)

        with open(csv_file_path,encoding="utf-8") as file:
            csv_file = pd.read_csv(file)

        self.glossary = translator.create_glossary_from_csv(
            name=glossary_name,
            source_lang=source_language,
            target_lang=target_language,
            csv_data=csv_file
        )
        return self.glossary.__dict__