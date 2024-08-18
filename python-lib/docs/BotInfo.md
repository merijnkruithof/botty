# BotInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**figure** | **str** |  | 
**gender** | **str** |  | 
**motto** | **str** |  | 
**sso_ticket** | **str** |  | 
**user_id** | **int** |  | 
**username** | **str** |  | 

## Example

```python
from openapi_client.models.bot_info import BotInfo

# TODO update the JSON string below
json = "{}"
# create an instance of BotInfo from a JSON string
bot_info_instance = BotInfo.from_json(json)
# print the JSON string representation of the object
print(BotInfo.to_json())

# convert the object into a dict
bot_info_dict = bot_info_instance.to_dict()
# create an instance of BotInfo from a dict
bot_info_from_dict = BotInfo.from_dict(bot_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


