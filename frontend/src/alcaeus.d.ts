declare module 'alcaeus' {
  export interface Resource {
    '@id': string;
    '@type': string | string[];
    [key: string]: any;
  }

  export interface HydraResponse<T extends Resource = Resource> {
    root: T;
    xhr?: any;
  }

  export interface AlcaeusClient {
    baseUri?: string;
    headers?: Record<string, string>;
    loadResource<T extends Resource = Resource>(url: string): Promise<HydraResponse<T>>;
  }

  export interface CreateOptions {
    dataset?: any;
    fetch?: typeof fetch;
    Headers?: typeof Headers;
    parsers?: any;
    rootSelectors?: any;
    datasetFactory?: any;
  }

  export function create(options: CreateOptions): AlcaeusClient;
}
