# .BotApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index**](BotApi.md#index) | **GET** /api/bots | 
[**show**](BotApi.md#show) | **GET** /api/bots/{ticket} | 


# **index**
> AvailableBots index(botsRequest)

Get all online bots including all user info.

### Example


```typescript
import {  } from '';
import * as fs from 'fs';

const configuration = .createConfiguration();
const apiInstance = new .BotApi(configuration);

let body:.BotApiIndexRequest = {
  // BotsRequest | Payload to request bots based on the hotel
  botsRequest: {
    hotel: "hotel_example",
  },
};

apiInstance.index(body).then((data:any) => {
  console.log('API called successfully. Returned data: ' + data);
}).catch((error:any) => console.error(error));
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **botsRequest** | **BotsRequest**| Payload to request bots based on the hotel |


### Return type

**AvailableBots**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List of available bots |  -  |
**400** | Bad request |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **show**
> AvailableBots show(showBotRequest)

Get a single bot\'s information.

### Example


```typescript
import {  } from '';
import * as fs from 'fs';

const configuration = .createConfiguration();
const apiInstance = new .BotApi(configuration);

let body:.BotApiShowRequest = {
  // string
  ticket: "ticket_example",
  // ShowBotRequest | Payload to request a single based on the hotel
  showBotRequest: {
    hotel: "hotel_example",
  },
};

apiInstance.show(body).then((data:any) => {
  console.log('API called successfully. Returned data: ' + data);
}).catch((error:any) => console.error(error));
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **showBotRequest** | **ShowBotRequest**| Payload to request a single based on the hotel |
 **ticket** | [**string**] |  | defaults to undefined


### Return type

**AvailableBots**

### Authorization

[api_key](README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | The requested bot |  -  |
**400** | Bad request |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)


