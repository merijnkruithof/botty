# AvailableBots


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bots** | [**List[BotInfo]**](BotInfo.md) |  | [optional] 

## Example

```python
from openapi_client.models.available_bots import AvailableBots

# TODO update the JSON string below
json = "{}"
# create an instance of AvailableBots from a JSON string
available_bots_instance = AvailableBots.from_json(json)
# print the JSON string representation of the object
print(AvailableBots.to_json())

# convert the object into a dict
available_bots_dict = available_bots_instance.to_dict()
# create an instance of AvailableBots from a dict
available_bots_from_dict = AvailableBots.from_dict(available_bots_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


