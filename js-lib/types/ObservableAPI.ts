import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration} from '../configuration'
import { Observable, of, from } from '../rxjsStub';
import {mergeMap, map} from  '../rxjsStub';
import { AvailableBots } from '../models/AvailableBots';
import { BotInfo } from '../models/BotInfo';
import { BotsRequest } from '../models/BotsRequest';
import { BulkUpdateResponse } from '../models/BulkUpdateResponse';
import { ShowBotRequest } from '../models/ShowBotRequest';

import { BotApiRequestFactory, BotApiResponseProcessor} from "../apis/BotApi";
export class ObservableBotApi {
    private requestFactory: BotApiRequestFactory;
    private responseProcessor: BotApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: BotApiRequestFactory,
        responseProcessor?: BotApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new BotApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new BotApiResponseProcessor();
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public indexWithHttpInfo(botsRequest: BotsRequest, _options?: Configuration): Observable<HttpInfo<AvailableBots>> {
        const requestContextPromise = this.requestFactory.index(botsRequest, _options);

        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (let middleware of this.configuration.middleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (let middleware of this.configuration.middleware) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.indexWithHttpInfo(rsp)));
            }));
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public index(botsRequest: BotsRequest, _options?: Configuration): Observable<AvailableBots> {
        return this.indexWithHttpInfo(botsRequest, _options).pipe(map((apiResponse: HttpInfo<AvailableBots>) => apiResponse.data));
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public showWithHttpInfo(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Observable<HttpInfo<AvailableBots>> {
        const requestContextPromise = this.requestFactory.show(ticket, showBotRequest, _options);

        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (let middleware of this.configuration.middleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (let middleware of this.configuration.middleware) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.showWithHttpInfo(rsp)));
            }));
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public show(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Observable<AvailableBots> {
        return this.showWithHttpInfo(ticket, showBotRequest, _options).pipe(map((apiResponse: HttpInfo<AvailableBots>) => apiResponse.data));
    }

}

import { BotControllerApiRequestFactory, BotControllerApiResponseProcessor} from "../apis/BotControllerApi";
export class ObservableBotControllerApi {
    private requestFactory: BotControllerApiRequestFactory;
    private responseProcessor: BotControllerApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: BotControllerApiRequestFactory,
        responseProcessor?: BotControllerApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new BotControllerApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new BotControllerApiResponseProcessor();
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public indexWithHttpInfo(botsRequest: BotsRequest, _options?: Configuration): Observable<HttpInfo<AvailableBots>> {
        const requestContextPromise = this.requestFactory.index(botsRequest, _options);

        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (let middleware of this.configuration.middleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (let middleware of this.configuration.middleware) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.indexWithHttpInfo(rsp)));
            }));
    }

    /**
     * Get all online bots including all user info.
     * @param botsRequest Payload to request bots based on the hotel
     */
    public index(botsRequest: BotsRequest, _options?: Configuration): Observable<AvailableBots> {
        return this.indexWithHttpInfo(botsRequest, _options).pipe(map((apiResponse: HttpInfo<AvailableBots>) => apiResponse.data));
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public showWithHttpInfo(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Observable<HttpInfo<AvailableBots>> {
        const requestContextPromise = this.requestFactory.show(ticket, showBotRequest, _options);

        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (let middleware of this.configuration.middleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (let middleware of this.configuration.middleware) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.showWithHttpInfo(rsp)));
            }));
    }

    /**
     * Get a single bot\'s information.
     * @param ticket 
     * @param showBotRequest Payload to request a single based on the hotel
     */
    public show(ticket: string, showBotRequest: ShowBotRequest, _options?: Configuration): Observable<AvailableBots> {
        return this.showWithHttpInfo(ticket, showBotRequest, _options).pipe(map((apiResponse: HttpInfo<AvailableBots>) => apiResponse.data));
    }

}
