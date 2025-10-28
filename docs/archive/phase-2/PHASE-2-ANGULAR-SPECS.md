# Phase 2: Frontend Angular - Spécifications Détaillées

**Version:** 0.5.0
**Date:** 2025-10-26
**Status:** 📋 Planification
**Durée estimée:** 3 semaines

---

## 🎯 Objectifs Phase 2

Créer un frontend Angular 17+ moderne qui consomme l'API Hydra hypermedia de Magneto-Serge, en utilisant **Alcaeus** pour la navigation automatique et **NgRx** pour la gestion d'état.

### Livrables Attendus

1. ✅ Application Angular 17+ standalone
2. ✅ Client Alcaeus configuré et opérationnel
3. ✅ State management NgRx avec entities
4. ✅ Interface utilisateur Material Design + Tailwind
5. ✅ Navigation hypermedia automatique
6. ✅ Tests E2E Cypress
7. ✅ Documentation utilisateur

---

## 📦 Stack Technique

### Frontend Core

- **Angular:** 17.x (standalone components)
- **TypeScript:** 5.x (strict mode)
- **RxJS:** 7.x (reactive programming)

### Hypermedia & État

- **Alcaeus:** `@wikibus/alcaeus` - Client Hydra/JSON-LD
- **NgRx:** State management (Store, Effects, Entity)
- **RDF.js:** Types pour Linked Data

### UI/UX

- **Angular Material:** Components Material Design
- **Tailwind CSS:** Utility-first styling
- **Angular CDK:** Composants avancés (virtual scroll, drag-drop)

### Testing

- **Jasmine/Karma:** Tests unitaires
- **Cypress:** Tests E2E
- **Testing Library:** Composants accessibles

---

## 🏗️ Architecture Application

### Structure Répertoires

```
magneto-ui/
├── src/
│   ├── app/
│   │   ├── core/                      # Services singleton
│   │   │   ├── hypermedia/
│   │   │   │   ├── alcaeus.service.ts
│   │   │   │   ├── api-entrypoint.service.ts
│   │   │   │   └── resource-cache.service.ts
│   │   │   ├── api/
│   │   │   │   └── magneto-api.service.ts
│   │   │   ├── state/
│   │   │   │   ├── cassettes/         # NgRx cassettes
│   │   │   │   ├── interactions/      # NgRx interactions
│   │   │   │   └── templates/         # NgRx templates
│   │   │   └── guards/
│   │   │       └── api-entrypoint.guard.ts
│   │   ├── features/                  # Modules fonctionnels
│   │   │   ├── cassettes/
│   │   │   │   ├── cassette-list/
│   │   │   │   ├── cassette-detail/
│   │   │   │   └── cassette-create/
│   │   │   ├── interactions/
│   │   │   │   ├── interaction-list/
│   │   │   │   └── interaction-detail/
│   │   │   ├── templates/
│   │   │   │   └── template-editor/
│   │   │   └── dashboard/
│   │   │       └── dashboard.component.ts
│   │   ├── shared/                    # Composants partagés
│   │   │   ├── components/
│   │   │   │   ├── pagination/
│   │   │   │   ├── loading-spinner/
│   │   │   │   └── error-display/
│   │   │   ├── directives/
│   │   │   │   └── hypermedia-link.directive.ts
│   │   │   ├── pipes/
│   │   │   │   ├── format-bytes.pipe.ts
│   │   │   │   └── format-date.pipe.ts
│   │   │   └── models/
│   │   │       ├── cassette.model.ts
│   │   │       ├── interaction.model.ts
│   │   │       └── hydra.model.ts
│   │   ├── app.component.ts
│   │   ├── app.config.ts
│   │   └── app.routes.ts
│   ├── assets/
│   ├── environments/
│   │   ├── environment.ts
│   │   └── environment.prod.ts
│   └── styles/
│       ├── tailwind.css
│       └── material-theme.scss
├── cypress/                           # Tests E2E
│   ├── e2e/
│   └── support/
├── angular.json
├── tailwind.config.js
└── tsconfig.json
```

---

## 🔌 Alcaeus Client - Configuration

### 1. Service Principal

```typescript
// src/app/core/hypermedia/alcaeus.service.ts
import { Injectable, inject } from '@angular/core';
import Alcaeus, { HydraResponse, Resource } from '@wikibus/alcaeus';
import { from, Observable } from 'rxjs';
import { environment } from '../../../environments/environment';

@Injectable({ providedIn: 'root' })
export class AlcaeusService {
  private readonly client = Alcaeus.withDefaults();

  constructor() {
    this.client.baseUri = environment.apiUrl;

    // Configure headers
    this.client.headers = {
      'Accept': 'application/ld+json',
      'Content-Type': 'application/ld+json'
    };
  }

  /**
   * Charge une ressource depuis l'API
   */
  loadResource<T extends Resource = Resource>(url: string): Observable<HydraResponse<T>> {
    return from(this.client.loadResource(url));
  }

  /**
   * Suit un lien hypermedia
   */
  followLink<T extends Resource = Resource>(
    resource: Resource,
    rel: string
  ): Observable<HydraResponse<T>> {
    const link = this.findLink(resource, rel);
    if (!link) {
      throw new Error(`Link relation "${rel}" not found`);
    }
    return this.loadResource<T>(link.href);
  }

  /**
   * Invoque une opération Hydra
   */
  invokeOperation<T = any>(
    operation: any,
    body?: any
  ): Observable<T> {
    return from(this.client.invokeOperation(operation, body));
  }

  /**
   * Trouve un lien dans une ressource
   */
  private findLink(resource: Resource, rel: string) {
    return resource.links?.find(link => link.rel === rel);
  }
}
```

### 2. API Entrypoint Service

```typescript
// src/app/core/hypermedia/api-entrypoint.service.ts
import { Injectable, inject } from '@angular/core';
import { Observable, shareReplay, tap } from 'rxjs';
import { AlcaeusService } from './alcaeus.service';
import { ApiDocumentation } from '../../shared/models/hydra.model';

@Injectable({ providedIn: 'root' })
export class ApiEntrypointService {
  private readonly alcaeus = inject(AlcaeusService);
  private entrypoint$: Observable<ApiDocumentation> | null = null;

  /**
   * Charge le point d'entrée API (auto-discovery)
   */
  loadEntrypoint(): Observable<ApiDocumentation> {
    if (!this.entrypoint$) {
      this.entrypoint$ = this.alcaeus.loadResource<ApiDocumentation>('/api').pipe(
        tap(response => console.log('API Entrypoint loaded:', response)),
        shareReplay(1)
      );
    }
    return this.entrypoint$;
  }

  /**
   * Récupère la collection de cassettes via hypermedia
   */
  getCassettesCollection(): Observable<any> {
    return this.loadEntrypoint().pipe(
      switchMap(entrypoint =>
        this.alcaeus.followLink(entrypoint.root, 'cassettes')
      )
    );
  }

  /**
   * Récupère la collection de templates via hypermedia
   */
  getTemplatesCollection(): Observable<any> {
    return this.loadEntrypoint().pipe(
      switchMap(entrypoint =>
        this.alcaeus.followLink(entrypoint.root, 'templates')
      )
    );
  }
}
```

---

## 🗂️ NgRx State Management

### 1. Cassettes State

```typescript
// src/app/core/state/cassettes/cassette.model.ts
export interface CassetteResource {
  '@id': string;
  '@type': string;
  name: string;
  version: string;
  recordedAt: string;
  interactionCount: number;
  sizeBytes: number;
  _links: {
    self: HydraLink;
    interactions: HydraLink;
    edit?: HydraLink;
    delete?: HydraLink;
  };
  'hydra:operation'?: HydraOperation[];
}

export interface HydraLink {
  href: string;
  templated?: boolean;
  title?: string;
}

export interface HydraOperation {
  '@type': string;
  'hydra:method': string;
  'hydra:expects'?: string;
  'hydra:returns'?: string;
  'hydra:title': string;
  'hydra:description'?: string;
}
```

```typescript
// src/app/core/state/cassettes/cassette.actions.ts
import { createActionGroup, emptyProps, props } from '@ngrx/store';
import { CassetteResource } from './cassette.model';

export const CassetteActions = createActionGroup({
  source: 'Cassette',
  events: {
    'Load Cassettes': emptyProps(),
    'Load Cassettes Success': props<{
      cassettes: CassetteResource[],
      totalItems: number,
      page: number
    }>(),
    'Load Cassettes Failure': props<{ error: string }>(),

    'Load Cassette': props<{ name: string }>(),
    'Load Cassette Success': props<{ cassette: CassetteResource }>(),
    'Load Cassette Failure': props<{ error: string }>(),

    'Delete Cassette': props<{ name: string }>(),
    'Delete Cassette Success': props<{ name: string }>(),
    'Delete Cassette Failure': props<{ error: string }>(),

    'Navigate to Page': props<{ page: number }>(),
  }
});
```

```typescript
// src/app/core/state/cassettes/cassette.reducer.ts
import { createReducer, on } from '@ngrx/store';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';
import { CassetteActions } from './cassette.actions';
import { CassetteResource } from './cassette.model';

export interface CassetteState extends EntityState<CassetteResource> {
  loading: boolean;
  error: string | null;
  totalItems: number;
  currentPage: number;
  pageSize: number;
}

export const adapter: EntityAdapter<CassetteResource> = createEntityAdapter<CassetteResource>({
  selectId: (cassette) => cassette.name,
  sortComparer: (a, b) => b.recordedAt.localeCompare(a.recordedAt),
});

const initialState: CassetteState = adapter.getInitialState({
  loading: false,
  error: null,
  totalItems: 0,
  currentPage: 1,
  pageSize: 20,
});

export const cassetteReducer = createReducer(
  initialState,
  on(CassetteActions.loadCassettes, (state) => ({
    ...state,
    loading: true,
    error: null,
  })),
  on(CassetteActions.loadCassettesSuccess, (state, { cassettes, totalItems, page }) =>
    adapter.setAll(cassettes, {
      ...state,
      loading: false,
      totalItems,
      currentPage: page,
    })
  ),
  on(CassetteActions.loadCassettesFailure, (state, { error }) => ({
    ...state,
    loading: false,
    error,
  })),
  on(CassetteActions.deleteCassetteSuccess, (state, { name }) =>
    adapter.removeOne(name, state)
  )
);
```

```typescript
// src/app/core/state/cassettes/cassette.effects.ts
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { map, catchError, switchMap, withLatestFrom } from 'rxjs/operators';
import { of } from 'rxjs';
import { Store } from '@ngrx/store';
import { CassetteActions } from './cassette.actions';
import { AlcaeusService } from '../../hypermedia/alcaeus.service';
import { selectCurrentPage, selectPageSize } from './cassette.selectors';

@Injectable()
export class CassetteEffects {
  private readonly actions$ = inject(Actions);
  private readonly alcaeus = inject(AlcaeusService);
  private readonly store = inject(Store);

  loadCassettes$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.loadCassettes, CassetteActions.navigateToPage),
      withLatestFrom(
        this.store.select(selectCurrentPage),
        this.store.select(selectPageSize)
      ),
      switchMap(([action, page, pageSize]) =>
        this.alcaeus.loadResource(`/api/cassettes?page=${page}&limit=${pageSize}`).pipe(
          map(response => {
            const collection = response.root;
            return CassetteActions.loadCassettesSuccess({
              cassettes: collection['hydra:member'],
              totalItems: collection['hydra:totalItems'],
              page,
            });
          }),
          catchError(error =>
            of(CassetteActions.loadCassettesFailure({
              error: error.message
            }))
          )
        )
      )
    )
  );

  loadCassette$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.loadCassette),
      switchMap(({ name }) =>
        this.alcaeus.loadResource(`/api/cassettes/${name}`).pipe(
          map(response =>
            CassetteActions.loadCassetteSuccess({
              cassette: response.root
            })
          ),
          catchError(error =>
            of(CassetteActions.loadCassetteFailure({
              error: error.message
            }))
          )
        )
      )
    )
  );
}
```

---

## 🎨 Composants UI

### 1. Cassette List Component

```typescript
// src/app/features/cassettes/cassette-list/cassette-list.component.ts
import { Component, OnInit, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Store } from '@ngrx/store';
import { Observable } from 'rxjs';
import { CassetteActions } from '../../../core/state/cassettes/cassette.actions';
import {
  selectAllCassettes,
  selectCassetteLoading,
  selectTotalItems,
  selectCurrentPage
} from '../../../core/state/cassettes/cassette.selectors';
import { CassetteResource } from '../../../core/state/cassettes/cassette.model';

@Component({
  selector: 'app-cassette-list',
  standalone: true,
  imports: [CommonModule, MatTableModule, MatPaginatorModule],
  template: `
    <div class="cassette-list-container">
      <h2>Cassettes Collection</h2>

      @if (loading$ | async) {
        <app-loading-spinner />
      } @else {
        <table mat-table [dataSource]="cassettes$ | async">
          <!-- Name Column -->
          <ng-container matColumnDef="name">
            <th mat-header-cell *matHeaderCellDef>Name</th>
            <td mat-cell *matCellDef="let cassette">
              <a [routerLink]="['/cassettes', cassette.name]">
                {{ cassette.name }}
              </a>
            </td>
          </ng-container>

          <!-- Interactions Column -->
          <ng-container matColumnDef="interactions">
            <th mat-header-cell *matHeaderCellDef>Interactions</th>
            <td mat-cell *matCellDef="let cassette">
              {{ cassette.interactionCount }}
            </td>
          </ng-container>

          <!-- Size Column -->
          <ng-container matColumnDef="size">
            <th mat-header-cell *matHeaderCellDef>Size</th>
            <td mat-cell *matCellDef="let cassette">
              {{ cassette.sizeBytes | formatBytes }}
            </td>
          </ng-container>

          <!-- Date Column -->
          <ng-container matColumnDef="recordedAt">
            <th mat-header-cell *matHeaderCellDef>Recorded</th>
            <td mat-cell *matCellDef="let cassette">
              {{ cassette.recordedAt | date:'medium' }}
            </td>
          </ng-container>

          <!-- Actions Column -->
          <ng-container matColumnDef="actions">
            <th mat-header-cell *matHeaderCellDef>Actions</th>
            <td mat-cell *matCellDef="let cassette">
              @if (cassette['hydra:operation']) {
                @for (op of cassette['hydra:operation']; track op['hydra:title']) {
                  <button mat-button (click)="invokeOperation(cassette, op)">
                    {{ op['hydra:title'] }}
                  </button>
                }
              }
            </td>
          </ng-container>

          <tr mat-header-row *matHeaderRowDef="displayedColumns"></tr>
          <tr mat-row *matRowDef="let row; columns: displayedColumns;"></tr>
        </table>

        <mat-paginator
          [length]="totalItems$ | async"
          [pageSize]="20"
          [pageSizeOptions]="[10, 20, 50, 100]"
          (page)="onPageChange($event)">
        </mat-paginator>
      }
    </div>
  `,
})
export class CassetteListComponent implements OnInit {
  private readonly store = inject(Store);

  cassettes$ = this.store.select(selectAllCassettes);
  loading$ = this.store.select(selectCassetteLoading);
  totalItems$ = this.store.select(selectTotalItems);
  currentPage$ = this.store.select(selectCurrentPage);

  displayedColumns = ['name', 'interactions', 'size', 'recordedAt', 'actions'];

  ngOnInit() {
    this.store.dispatch(CassetteActions.loadCassettes());
  }

  onPageChange(event: PageEvent) {
    this.store.dispatch(CassetteActions.navigateToPage({
      page: event.pageIndex + 1
    }));
  }

  invokeOperation(cassette: CassetteResource, operation: any) {
    // Invoke Hydra operation via Alcaeus
    console.log('Invoking operation:', operation, 'on', cassette);
  }
}
```

---

## 🧪 Tests

### 1. Tests Unitaires (Jasmine)

```typescript
// src/app/core/hypermedia/alcaeus.service.spec.ts
import { TestBed } from '@angular/core/testing';
import { AlcaeusService } from './alcaeus.service';

describe('AlcaeusService', () => {
  let service: AlcaeusService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AlcaeusService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should load resource with correct URL', (done) => {
    service.loadResource('/api').subscribe(response => {
      expect(response).toBeDefined();
      done();
    });
  });
});
```

### 2. Tests E2E (Cypress)

```typescript
// cypress/e2e/cassettes.cy.ts
describe('Cassettes Feature', () => {
  beforeEach(() => {
    cy.visit('/cassettes');
  });

  it('should display cassettes list', () => {
    cy.get('table').should('exist');
    cy.get('table tbody tr').should('have.length.greaterThan', 0);
  });

  it('should navigate to cassette detail', () => {
    cy.get('table tbody tr').first().find('a').click();
    cy.url().should('include', '/cassettes/');
    cy.get('h2').should('contain', 'Cassette Detail');
  });

  it('should follow hypermedia links', () => {
    cy.get('table tbody tr').first().find('a').click();
    cy.get('button').contains('View Interactions').click();
    cy.url().should('include', '/interactions');
  });

  it('should handle pagination', () => {
    cy.get('mat-paginator').should('exist');
    cy.get('[aria-label="Next page"]').click();
    cy.url().should('include', 'page=2');
  });
});
```

---

## 📋 Checklist Implémentation

### Semaine 4: Setup & Configuration
- [ ] Créer projet Angular 17 standalone
- [ ] Installer dépendances (Alcaeus, NgRx, Material)
- [ ] Configurer Tailwind CSS
- [ ] Setup structure répertoires
- [ ] Configurer Alcaeus service
- [ ] Implémenter API entrypoint service
- [ ] Tests unitaires services core

### Semaine 5: State Management & Components
- [ ] Setup NgRx store (cassettes, interactions, templates)
- [ ] Implémenter actions, reducers, effects
- [ ] Créer selectors
- [ ] Composant liste cassettes
- [ ] Composant détail cassette
- [ ] Composant liste interactions
- [ ] Navigation hypermedia automatique
- [ ] Tests unitaires composants

### Semaine 6: UI Polish & Tests
- [ ] Material Design theming
- [ ] Pagination component réutilisable
- [ ] Loading states & error handling
- [ ] Responsive design (mobile, tablet, desktop)
- [ ] Tests E2E Cypress (parcours complets)
- [ ] Performance optimization (lazy loading)
- [ ] Documentation utilisateur

---

## 🚀 Commandes Utiles

```bash
# Créer projet
ng new magneto-ui --standalone --routing --style=scss --ssr=false

# Installer dépendances
npm install @wikibus/alcaeus @rdfjs/types
npm install @ngrx/store @ngrx/effects @ngrx/entity
npm install @angular/material @angular/cdk
npm install -D tailwindcss postcss autoprefixer

# Développement
ng serve                    # Dev server
ng test                     # Tests unitaires
ng build                    # Build production
npx cypress open            # Tests E2E

# Générer composants
ng generate component features/cassettes/cassette-list --standalone
ng generate service core/hypermedia/alcaeus
```

---

## 📚 Ressources

### Documentation Officielle
- **Angular:** https://angular.dev/
- **Alcaeus:** https://github.com/wikibus/Alcaeus
- **NgRx:** https://ngrx.io/
- **Material Design:** https://material.angular.io/
- **Hydra Spec:** https://www.hydra-cg.com/spec/latest/core/

### Exemples
- **Alcaeus Demo:** https://github.com/wikibus/Alcaeus/tree/master/examples
- **NgRx Example App:** https://github.com/ngrx/platform/tree/main/projects/example-app

---

**Prochaine étape**: Commencer l'implémentation après validation CI Phase 1 ✅
