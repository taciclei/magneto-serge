/**
 * Type definitions for alcaeus
 * Alcaeus has ESM issues with TypeScript, so we provide minimal types
 */

declare module 'alcaeus' {
  export interface IResource {
    [key: string]: any;
    '@id'?: string;
    '@type'?: string | string[];
    operations?: IOperation[];
  }

  export interface IHydraResponse {
    root: IResource;
    xhr?: XMLHttpRequest;
  }

  export interface IOperation {
    '@id'?: string;
    '@type'?: string;
    method: string;
    expects?: string;
    returns?: string;
    target?: IResource;
    invoke(body?: any): Promise<IHydraResponse>;
  }

  export interface IHydraClient {
    loadResource(url: string): Promise<IHydraResponse>;
    defaultHeaders: Record<string, string>;
  }

  const Alcaeus: IHydraClient;
  export default Alcaeus;
}
