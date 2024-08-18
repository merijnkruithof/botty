# OpenAPI\Client\BotApi

All URIs are relative to http://localhost, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**index()**](BotApi.md#index) | **GET** /api/bots |  |
| [**show()**](BotApi.md#show) | **GET** /api/bots/{ticket} |  |


## `index()`

```php
index($bots_request): \OpenAPI\Client\Model\AvailableBots
```



Get all online bots including all user info.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = OpenAPI\Client\Configuration::getDefaultConfiguration()->setApiKey('x-auth-token', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = OpenAPI\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-auth-token', 'Bearer');


$apiInstance = new OpenAPI\Client\Api\BotApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$bots_request = new \OpenAPI\Client\Model\BotsRequest(); // \OpenAPI\Client\Model\BotsRequest | Payload to request bots based on the hotel

try {
    $result = $apiInstance->index($bots_request);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling BotApi->index: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **bots_request** | [**\OpenAPI\Client\Model\BotsRequest**](../Model/BotsRequest.md)| Payload to request bots based on the hotel | |

### Return type

[**\OpenAPI\Client\Model\AvailableBots**](../Model/AvailableBots.md)

### Authorization

[api_key](../../README.md#api_key)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `show()`

```php
show($ticket, $show_bot_request): \OpenAPI\Client\Model\AvailableBots
```



Get a single bot's information.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = OpenAPI\Client\Configuration::getDefaultConfiguration()->setApiKey('x-auth-token', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = OpenAPI\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-auth-token', 'Bearer');


$apiInstance = new OpenAPI\Client\Api\BotApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$ticket = 'ticket_example'; // string
$show_bot_request = new \OpenAPI\Client\Model\ShowBotRequest(); // \OpenAPI\Client\Model\ShowBotRequest | Payload to request a single based on the hotel

try {
    $result = $apiInstance->show($ticket, $show_bot_request);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling BotApi->show: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **ticket** | **string**|  | |
| **show_bot_request** | [**\OpenAPI\Client\Model\ShowBotRequest**](../Model/ShowBotRequest.md)| Payload to request a single based on the hotel | |

### Return type

[**\OpenAPI\Client\Model\AvailableBots**](../Model/AvailableBots.md)

### Authorization

[api_key](../../README.md#api_key)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)
