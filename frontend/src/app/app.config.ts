import { ApplicationConfig, provideZoneChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideHttpClient } from '@angular/common/http';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { provideStore } from '@ngrx/store';
import { provideEffects } from '@ngrx/effects';
import { provideStoreDevtools } from '@ngrx/store-devtools';

import { routes } from './app.routes';
import { cassettesReducer } from './features/cassettes/state/cassette.reducer';
import { CassetteEffects } from './features/cassettes/state/cassette.effects';
import { environment } from '../environments/environment';

/**
 * Configuration globale de l'application Angular
 *
 * Configure:
 * - Router
 * - HTTP Client
 * - Animations Material
 * - NgRx Store + Effects + DevTools
 */
export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    provideHttpClient(),
    provideAnimationsAsync(),
    provideStore({ cassettes: cassettesReducer }),
    provideEffects([CassetteEffects]),
    provideStoreDevtools({
      maxAge: 25,
      logOnly: environment.production,
      autoPause: true,
      trace: false,
      traceLimit: 75
    })
  ]
};
