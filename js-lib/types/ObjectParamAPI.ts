import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration} from '../configuration'

import { AvailableBots } from '../models/AvailableBots';
import { BotInfo } from '../models/BotInfo';
import { BotsRequest } from '../models/BotsRequest';
import { BulkUpdateResponse } from '../models/BulkUpdateResponse';
import { ShowBotRequest } from '../models/ShowBotRequest';

import { ObservableBotApi } from "./ObservableAPI";
import { BotApiRequestFactory, BotApiResponseProcessor} from "../apis/BotApi";

export interface BotApiIndexRequest {
    /**
     * Payload to request bots based on the hotel
     * @type BotsRequest
     * @memberof BotApiindex
     */
    botsRequest: BotsRequest
}

export interface BotApiShowRequest {
    /**
     * 
     * @type string
     * @memberof BotApishow
     */
    ticket: string
    /**
     * Payload to request a single based on the hotel
     * @type ShowBotRequest
     * @memberof BotApishow
     */
    showBotRequest: ShowBotRequest
}

export class ObjectBotApi {
    private api: ObservableBotApi

    public constructor(configuration: Configuration, requestFactory?: BotApiRequestFactory, responseProcessor?: BotApiResponseProcessor) {
        this.api = new ObservableBotApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Get all online bots including all user info.
     * @param param the request object
     */
    public indexWithHttpInfo(param: BotApiIndexRequest, options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        return this.api.indexWithHttpInfo(param.botsRequest,  options).toPromise();
    }

    /**
     * Get all online bots including all user info.
     * @param param the request object
     */
    public index(param: BotApiIndexRequest, options?: Configuration): Promise<AvailableBots> {
        return this.api.index(param.botsRequest,  options).toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param param the request object
     */
    public showWithHttpInfo(param: BotApiShowRequest, options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        return this.api.showWithHttpInfo(param.ticket, param.showBotRequest,  options).toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param param the request object
     */
    public show(param: BotApiShowRequest, options?: Configuration): Promise<AvailableBots> {
        return this.api.show(param.ticket, param.showBotRequest,  options).toPromise();
    }

}

import { ObservableBotControllerApi } from "./ObservableAPI";
import { BotControllerApiRequestFactory, BotControllerApiResponseProcessor} from "../apis/BotControllerApi";

export interface BotControllerApiIndexRequest {
    /**
     * Payload to request bots based on the hotel
     * @type BotsRequest
     * @memberof BotControllerApiindex
     */
    botsRequest: BotsRequest
}

export interface BotControllerApiShowRequest {
    /**
     * 
     * @type string
     * @memberof BotControllerApishow
     */
    ticket: string
    /**
     * Payload to request a single based on the hotel
     * @type ShowBotRequest
     * @memberof BotControllerApishow
     */
    showBotRequest: ShowBotRequest
}

export class ObjectBotControllerApi {
    private api: ObservableBotControllerApi

    public constructor(configuration: Configuration, requestFactory?: BotControllerApiRequestFactory, responseProcessor?: BotControllerApiResponseProcessor) {
        this.api = new ObservableBotControllerApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Get all online bots including all user info.
     * @param param the request object
     */
    public indexWithHttpInfo(param: BotControllerApiIndexRequest, options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        return this.api.indexWithHttpInfo(param.botsRequest,  options).toPromise();
    }

    /**
     * Get all online bots including all user info.
     * @param param the request object
     */
    public index(param: BotControllerApiIndexRequest, options?: Configuration): Promise<AvailableBots> {
        return this.api.index(param.botsRequest,  options).toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param param the request object
     */
    public showWithHttpInfo(param: BotControllerApiShowRequest, options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        return this.api.showWithHttpInfo(param.ticket, param.showBotRequest,  options).toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param param the request object
     */
    public show(param: BotControllerApiShowRequest, options?: Configuration): Promise<AvailableBots> {
        return this.api.show(param.ticket, param.showBotRequest,  options).toPromise();
    }

}
