# Phase 3.1: Recherche et Filtres

**Date:** À définir
**Durée estimée:** 5-7 jours
**Statut:** 📋 Planification
**Priorité:** 🟡 Moyenne

---

## 📋 Objectif

Implémenter une recherche full-text et des filtres avancés pour faciliter la navigation dans une grande collection de cassettes.

---

## 🎯 User Stories

### US-3.1.1: Recherche Full-Text
**En tant qu'** utilisateur
**Je veux** pouvoir rechercher des cassettes par nom ou description
**Afin de** trouver rapidement une cassette spécifique

**Critères d'acceptation:**
- [ ] Barre de recherche visible dans toolbar
- [ ] Input avec debounce 300ms
- [ ] Recherche dans nom ET description
- [ ] Résultats affichés en temps réel
- [ ] Indicateur "X résultats trouvés"

### US-3.1.2: Filtres par Type
**En tant qu'** utilisateur
**Je veux** filtrer les cassettes par type (HTTP/WebSocket)
**Afin de** voir uniquement les cassettes d'un type spécifique

**Critères d'acceptation:**
- [ ] Menu déroulant "Type" avec options: Tous, HTTP, WebSocket
- [ ] Sélection applique filtre immédiatement
- [ ] Chip affiché pour filtre actif
- [ ] Click sur chip retire le filtre

### US-3.1.3: Tri par Colonne
**En tant qu'** utilisateur
**Je veux** trier les cassettes par nom ou date
**Afin de** organiser l'affichage selon mes besoins

**Critères d'acceptation:**
- [ ] Click sur en-tête "Nom" → tri par nom
- [ ] Click sur en-tête "Date" → tri par date
- [ ] Indicateur ↑↓ pour ordre croissant/décroissant
- [ ] Toggle ascendant/descendant sur re-click

---

## 🏗️ Architecture Backend

### Endpoint Amélioré

#### GET /api/cassettes avec Query Params

```http
GET /api/cassettes?search=user&type=http&sort=date&order=desc HTTP/1.1
Host: localhost:8889
```

**Query Parameters:**
- `search` (string, optional): Terme de recherche (nom ou description)
- `type` (string, optional): `http`, `websocket`, ou `all` (default: `all`)
- `from` (ISO8601, optional): Date minimum
- `to` (ISO8601, optional): Date maximum
- `sort` (string, optional): `name` ou `date` (default: `name`)
- `order` (string, optional): `asc` ou `desc` (default: `asc`)
- `page` (number, optional): Numéro de page (default: 1)
- `per_page` (number, optional): Items par page (default: 10)

**Réponse 200 OK:**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Collection",
  "@id": "http://localhost:8889/api/cassettes?search=user&type=http",
  "hydra:totalItems": 5,
  "hydra:member": [...],
  "hydra:view": {
    "@type": "hydra:PartialCollectionView",
    "hydra:first": "http://localhost:8889/api/cassettes?page=1&per_page=10",
    "hydra:last": "http://localhost:8889/api/cassettes?page=1&per_page=10"
  },
  "hydra:search": {
    "@type": "hydra:IriTemplate",
    "hydra:template": "http://localhost:8889/api/cassettes{?search,type,sort,order}",
    "hydra:mapping": [
      {
        "@type": "hydra:IriTemplateMapping",
        "hydra:variable": "search",
        "hydra:property": "name or description",
        "hydra:required": false
      },
      {
        "@type": "hydra:IriTemplateMapping",
        "hydra:variable": "type",
        "hydra:property": "interaction type",
        "hydra:required": false
      }
    ]
  }
}
```

---

## 💻 Implémentation Backend

### Code Backend - Handler GET Amélioré

**`src/api/handlers/cassettes.rs` (modifications):**

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CassettesQueryParams {
    pub search: Option<String>,
    pub type_: Option<String>, // "http", "websocket", "all"
    pub from: Option<String>,   // ISO8601 date
    pub to: Option<String>,     // ISO8601 date
    pub sort: Option<String>,   // "name", "date"
    pub order: Option<String>,  // "asc", "desc"
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

pub async fn get_cassettes(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<CassettesQueryParams>,
) -> Result<Json<HydraCollection>, StatusCode> {
    // Charger toutes les cassettes
    let mut cassettes = load_all_cassettes(&state.cassette_dir)?;

    // Filtrer par recherche
    if let Some(search_term) = params.search {
        let term_lower = search_term.to_lowercase();
        cassettes.retain(|c| {
            c.name.to_lowercase().contains(&term_lower)
                || c.description.to_lowercase().contains(&term_lower)
        });
    }

    // Filtrer par type
    if let Some(type_filter) = params.type_ {
        if type_filter != "all" {
            cassettes.retain(|c| c.has_interaction_type(&type_filter));
        }
    }

    // Filtrer par date
    if let Some(from_date) = params.from {
        let from = chrono::DateTime::parse_from_rfc3339(&from_date).ok()?;
        cassettes.retain(|c| c.recorded_at >= from);
    }

    // Trier
    let sort_by = params.sort.as_deref().unwrap_or("name");
    let order = params.order.as_deref().unwrap_or("asc");

    match sort_by {
        "name" => cassettes.sort_by(|a, b| a.name.cmp(&b.name)),
        "date" => cassettes.sort_by(|a, b| a.recorded_at.cmp(&b.recorded_at)),
        _ => {}
    }

    if order == "desc" {
        cassettes.reverse();
    }

    // Paginer
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let total = cassettes.len();
    let start = (page - 1) * per_page;
    let end = std::cmp::min(start + per_page, total);

    let paginated = cassettes[start..end].to_vec();

    // Construire réponse Hydra
    build_hydra_collection(paginated, total, page, per_page, &params, &state.base_url)
}
```

---

## 🅰️ Implémentation Frontend

### Fichiers à Créer

```
frontend/src/app/features/cassettes/components/
├── cassette-search-bar/
│   ├── cassette-search-bar.component.ts
│   ├── cassette-search-bar.component.html
│   └── cassette-search-bar.component.scss
└── cassette-filters/
    ├── cassette-filters.component.ts
    ├── cassette-filters.component.html
    └── cassette-filters.component.scss
```

### Code Frontend - Search Bar Component

**`cassette-search-bar.component.ts`:**

```typescript
import { Component, OnInit, OnDestroy, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { Store } from '@ngrx/store';
import { Subject } from 'rxjs';
import { debounceTime, distinctUntilChanged, takeUntil } from 'rxjs/operators';
import * as CassetteActions from '../../store/cassette.actions';

@Component({
  selector: 'app-cassette-search-bar',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatIconModule,
  ],
  template: `
    <mat-form-field appearance="outline" class="search-field">
      <mat-label>Search cassettes</mat-label>
      <input
        matInput
        [(ngModel)]="searchTerm"
        (ngModelChange)="onSearchChange($event)"
        placeholder="Search by name or description"
      >
      <mat-icon matPrefix>search</mat-icon>
      <button
        mat-icon-button
        matSuffix
        *ngIf="searchTerm"
        (click)="clearSearch()"
      >
        <mat-icon>close</mat-icon>
      </button>
    </mat-form-field>
  `,
  styles: [`
    .search-field {
      width: 100%;
      max-width: 400px;
    }
  `]
})
export class CassetteSearchBarComponent implements OnInit, OnDestroy {
  private store = inject(Store);
  private destroy$ = new Subject<void>();
  private searchSubject$ = new Subject<string>();

  searchTerm = '';

  ngOnInit(): void {
    this.searchSubject$
      .pipe(
        debounceTime(300),
        distinctUntilChanged(),
        takeUntil(this.destroy$)
      )
      .subscribe(term => {
        this.store.dispatch(CassetteActions.setSearchTerm({ searchTerm: term }));
        this.store.dispatch(CassetteActions.loadCassettes());
      });
  }

  onSearchChange(term: string): void {
    this.searchSubject$.next(term);
  }

  clearSearch(): void {
    this.searchTerm = '';
    this.onSearchChange('');
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }
}
```

### NgRx Actions (additions)

**`cassette.actions.ts`:**

```typescript
// SEARCH
export const setSearchTerm = createAction(
  '[Cassette] Set Search Term',
  props<{ searchTerm: string }>()
);

export const setTypeFilter = createAction(
  '[Cassette] Set Type Filter',
  props<{ type: 'all' | 'http' | 'websocket' }>()
);

export const setSortBy = createAction(
  '[Cassette] Set Sort By',
  props<{ sortBy: 'name' | 'date'; order: 'asc' | 'desc' }>()
);

export const clearFilters = createAction(
  '[Cassette] Clear Filters'
);
```

### NgRx Reducer (modifications)

**`cassette.reducer.ts`:**

```typescript
export interface CassetteState {
  cassettes: Cassette[];
  totalItems: number;
  loading: boolean;
  error: string | null;

  // Filters
  searchTerm: string;
  typeFilter: 'all' | 'http' | 'websocket';
  sortBy: 'name' | 'date';
  sortOrder: 'asc' | 'desc';
}

const initialState: CassetteState = {
  cassettes: [],
  totalItems: 0,
  loading: false,
  error: null,

  searchTerm: '',
  typeFilter: 'all',
  sortBy: 'name',
  sortOrder: 'asc',
};

export const cassetteReducer = createReducer(
  initialState,

  on(CassetteActions.setSearchTerm, (state, { searchTerm }) => ({
    ...state,
    searchTerm,
  })),

  on(CassetteActions.setTypeFilter, (state, { type }) => ({
    ...state,
    typeFilter: type,
  })),

  on(CassetteActions.setSortBy, (state, { sortBy, order }) => ({
    ...state,
    sortBy,
    sortOrder: order,
  })),

  on(CassetteActions.clearFilters, state => ({
    ...state,
    searchTerm: '',
    typeFilter: 'all',
    sortBy: 'name',
    sortOrder: 'asc',
  }))
);
```

### AlcaeusService (modifications)

**`alcaeus.service.ts`:**

```typescript
loadCassettes(params: {
  search?: string;
  type?: string;
  sort?: string;
  order?: string;
  page?: number;
  per_page?: number;
}): Observable<HydraResponse<HydraCollection>> {
  const queryString = new URLSearchParams(
    Object.entries(params)
      .filter(([_, value]) => value !== undefined)
      .map(([key, value]) => [key, String(value)])
  ).toString();

  const url = `/api/cassettes${queryString ? '?' + queryString : ''}`;
  return this.loadResource<HydraCollection>(url);
}
```

---

## 🧪 Tests

### Tests Backend

```rust
#[tokio::test]
async fn test_search_by_name() {
    let app = test_app().await;

    let response = app.get("/api/cassettes?search=user").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body: HydraCollection = response.json().await;
    assert!(body.member.iter().all(|c| c.name.contains("user")));
}

#[tokio::test]
async fn test_filter_by_type() {
    let app = test_app().await;

    let response = app.get("/api/cassettes?type=http").await;
    let body: HydraCollection = response.json().await;

    assert!(body.member.iter().all(|c| c.has_http_interactions()));
}

#[tokio::test]
async fn test_sort_by_date_desc() {
    let app = test_app().await;

    let response = app.get("/api/cassettes?sort=date&order=desc").await;
    let body: HydraCollection = response.json().await;

    // Vérifier ordre décroissant
    for i in 0..body.member.len() - 1 {
        assert!(body.member[i].recorded_at >= body.member[i + 1].recorded_at);
    }
}
```

### Tests Frontend

```typescript
describe('CassetteSearchBarComponent', () => {
  it('should debounce search input', fakeAsync(() => {
    const store = TestBed.inject(Store);
    const dispatchSpy = jest.spyOn(store, 'dispatch');

    component.searchTerm = 'test';
    component.onSearchChange('test');

    tick(200); // Avant debounce
    expect(dispatchSpy).not.toHaveBeenCalled();

    tick(100); // Après debounce (300ms)
    expect(dispatchSpy).toHaveBeenCalledWith(
      CassetteActions.setSearchTerm({ searchTerm: 'test' })
    );
  }));
});
```

---

## ✅ Critères d'Achèvement

### Backend
- [ ] Endpoint supporte query params: search, type, sort, order
- [ ] Recherche full-text fonctionne (nom + description)
- [ ] Filtres par type fonctionnent
- [ ] Tri par nom/date fonctionne
- [ ] Spec Hydra inclut hydra:search template
- [ ] Tests unitaires passent (10+ tests)

### Frontend
- [ ] Barre de recherche avec debounce fonctionne
- [ ] Filtres type affichent chips
- [ ] Tri par colonne fonctionne (↑↓ indicateur)
- [ ] URL query params synchronisés
- [ ] Tests unitaires passent (10+ tests)

---

**Auteur:** Claude Code + Équipe Magnéto-Serge
**Date:** 2025-10-26
