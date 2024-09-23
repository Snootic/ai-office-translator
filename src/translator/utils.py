import deepl
from openai import OpenAI

class DeeplAccount():
    def __init__(self) -> None:
        self.API_KEY: str
        
    def main(self,api_key: str):
        self.API_KEY = api_key

    def check_usage(self) -> list:
        usage = deepl.Translator(self.API_KEY).get_usage()
        
        usage_dict = {}
        
        if usage.character.valid:
            usage_dict["used_characters"] = usage.character.count
            usage_dict["characters_limit"] = usage.character.limit
            usage_dict["characters_limit_reached"] = usage.character.limit_reached
        
        if usage.document.valid:
            usage_dict["used_documents"] = usage.document.count
            usage_dict["documents_limit"] = usage.document.limit
            usage_dict["documents_limit_reached"] = usage.document.limit_reached

        return usage_dict
    
    def get_languages(self,type: str):
        """
            type is either source or target
        """
        
        languages_raw = []
        
        if type == "source":
            languages_raw = deepl.Translator(self.API_KEY).get_source_languages()
            
        if type == "target":
            languages_raw = deepl.Translator(self.API_KEY).get_target_languages()
        
        languages = [language.__dict__ for language in languages_raw]
            
        return languages

class GPTAccount():
    def __init__(self) -> None:
        self.API_KEY: str
        self.CLIENT: OpenAI
        
    def main(self,api_key: str):
        self.API_KEY = api_key
        self.CLIENT = OpenAI(api_key=self.API_KEY)
    
    def models(self):
        gpt_models = self.CLIENT.models.list()
        
        models = []
        for model in gpt_models:
            if model.id.__contains__("gpt-4o"):
                models.append(model.id)

        gpt_models = {"gpt_models": models}
                
        return gpt_models
    
    def account_billing():
        #TODO
        pass

gpt = GPTAccount()
deep = DeeplAccount()