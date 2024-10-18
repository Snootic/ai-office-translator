import deepl
from openai import OpenAI

class DeeplAccount():
    def __init__(self,api_key: str) -> None:
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
    def __init__(self,api_key: str) -> None:
        self.API_KEY = api_key
        self.CLIENT = OpenAI(api_key=self.API_KEY)

    def models(self):
        gpt_models = self.CLIENT.models.list()
        
        models = []
        for model in gpt_models:
            # filtering only gpt-4o and gpt-4o-mini because theyre more than sufficient
            # and to not confuse end user with a ton of models
            # when gpt-4o become deprecated, replace this with the most cost-benefit model at the time
            if model.id == "gpt-4o" or model.id == "gpt-4o-mini":
                models.append(model.id)

        gpt_models = {"gpt_models": models}
                
        return gpt_models
    
    def account_billing():
        #not sure if openai provides a way to check the account billing from the api
        #if not this function will never be made kek
        #TODO
        pass
