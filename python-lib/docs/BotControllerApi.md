# openapi_client.BotControllerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index**](BotControllerApi.md#index) | **GET** /api/bots | 
[**show**](BotControllerApi.md#show) | **GET** /api/bots/{ticket} | 


# **index**
> AvailableBots index(bots_request)



Get all online bots including all user info.

### Example

* Api Key Authentication (api_key):

```python
import openapi_client
from openapi_client.models.available_bots import AvailableBots
from openapi_client.models.bots_request import BotsRequest
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.BotControllerApi(api_client)
    bots_request = openapi_client.BotsRequest() # BotsRequest | Payload to request bots based on the hotel

    try:
        api_response = api_instance.index(bots_request)
        print("The response of BotControllerApi->index:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling BotControllerApi->index: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **bots_request** | [**BotsRequest**](BotsRequest.md)| Payload to request bots based on the hotel | 

### Return type

[**AvailableBots**](AvailableBots.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List of available bots |  -  |
**400** | Bad request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **show**
> AvailableBots show(ticket, show_bot_request)



Get a single bot's information.

### Example

* Api Key Authentication (api_key):

```python
import openapi_client
from openapi_client.models.available_bots import AvailableBots
from openapi_client.models.show_bot_request import ShowBotRequest
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.BotControllerApi(api_client)
    ticket = 'ticket_example' # str | 
    show_bot_request = openapi_client.ShowBotRequest() # ShowBotRequest | Payload to request a single based on the hotel

    try:
        api_response = api_instance.show(ticket, show_bot_request)
        print("The response of BotControllerApi->show:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling BotControllerApi->show: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ticket** | **str**|  | 
 **show_bot_request** | [**ShowBotRequest**](ShowBotRequest.md)| Payload to request a single based on the hotel | 

### Return type

[**AvailableBots**](AvailableBots.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The requested bot |  -  |
**400** | Bad request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

