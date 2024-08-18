import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration} from '../configuration'

import { AvailableBots } from '../models/AvailableBots';
import { BotInfo } from '../models/BotInfo';
import { BotsRequest } from '../models/BotsRequest';
import { BulkUpdateResponse } from '../models/BulkUpdateResponse';
import { ShowBotRequest } from '../models/ShowBotRequest';
import { ObservableBotApi } from './ObservableAPI';

import { BotApiRequestFactory, BotApiResponseProcessor} from "../apis/BotApi";
export class PromiseBotApi {
    private api: ObservableBotApi

    public constructor(
        configuration: Configuration,
        requestFactory?: BotApiRequestFactory,
        responseProcessor?: BotApiResponseProcessor
    ) {
        this.api = new ObservableBotApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public indexWithHttpInfo(botsRequest: BotsRequest, _options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        const result = this.api.indexWithHttpInfo(botsRequest, _options);
        return result.toPromise();
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public index(botsRequest: BotsRequest, _options?: Configuration): Promise<AvailableBots> {
        const result = this.api.index(botsRequest, _options);
        return result.toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public showWithHttpInfo(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        const result = this.api.showWithHttpInfo(ticket, showBotRequest, _options);
        return result.toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public show(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Promise<AvailableBots> {
        const result = this.api.show(ticket, showBotRequest, _options);
        return result.toPromise();
    }


}



import { ObservableBotControllerApi } from './ObservableAPI';

import { BotControllerApiRequestFactory, BotControllerApiResponseProcessor} from "../apis/BotControllerApi";
export class PromiseBotControllerApi {
    private api: ObservableBotControllerApi

    public constructor(
        configuration: Configuration,
        requestFactory?: BotControllerApiRequestFactory,
        responseProcessor?: BotControllerApiResponseProcessor
    ) {
        this.api = new ObservableBotControllerApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public indexWithHttpInfo(botsRequest: BotsRequest, _options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        const result = this.api.indexWithHttpInfo(botsRequest, _options);
        return result.toPromise();
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public index(botsRequest: BotsRequest, _options?: Configuration): Promise<AvailableBots> {
        const result = this.api.index(botsRequest, _options);
        return result.toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public showWithHttpInfo(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Promise<HttpInfo<AvailableBots>> {
        const result = this.api.showWithHttpInfo(ticket, showBotRequest, _options);
        return result.toPromise();
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public show(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Promise<AvailableBots> {
        const result = this.api.show(ticket, showBotRequest, _options);
        return result.toPromise();
    }


}



