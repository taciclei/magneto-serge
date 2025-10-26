import { Routes } from '@angular/router';
import { CassetteListComponent } from './features/cassettes/components/cassette-list/cassette-list.component';
import { CassetteDetailComponent } from './features/cassettes/components/cassette-detail/cassette-detail.component';

/**
 * Configuration du routing Angular
 *
 * Routes principales:
 * - / : Redirige vers /cassettes
 * - /cassettes : Liste paginée des cassettes
 * - /cassettes/:name : Détails d'une cassette
 */
export const routes: Routes = [
  {
    path: '',
    redirectTo: '/cassettes',
    pathMatch: 'full'
  },
  {
    path: 'cassettes',
    component: CassetteListComponent,
    title: 'Cassettes - Magnéto-Serge'
  },
  {
    path: 'cassettes/:name',
    component: CassetteDetailComponent,
    title: 'Détails cassette - Magnéto-Serge'
  },
  {
    path: '**',
    redirectTo: '/cassettes'
  }
];
