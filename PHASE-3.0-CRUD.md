# Phase 3.0: Actions CRUD sur Cassettes

**Date:** À définir
**Durée estimée:** 5-7 jours
**Statut:** 📋 Planification
**Priorité:** 🔴 Haute

---

## 📋 Objectif

Implémenter les opérations CRUD complètes (Create, Read, Update, Delete) pour les cassettes via l'API Hydra et le frontend Angular.

**Actuellement:** Seule la lecture (GET) est implémentée
**Après Phase 3.0:** Création, modification et suppression complètes

---

## 🎯 User Stories

### US-3.0.1: Créer une Cassette
**En tant qu'** utilisateur
**Je veux** pouvoir créer une nouvelle cassette vide
**Afin de** commencer à enregistrer des interactions HTTP/WebSocket

**Critères d'acceptation:**
- [ ] Bouton "Nouvelle Cassette" visible dans toolbar
- [ ] Click ouvre un dialog Material avec formulaire
- [ ] Champs: Nom (requis), Description (optionnelle), Mode (auto/record/replay)
- [ ] Validation temps réel du nom (alphanumérique, unique)
- [ ] Soumission envoie POST /api/cassettes
- [ ] Cassette apparaît dans liste après création
- [ ] Toast de confirmation affiché

### US-3.0.2: Supprimer une Cassette
**En tant qu'** utilisateur
**Je veux** pouvoir supprimer une cassette existante
**Afin de** nettoyer les cassettes obsolètes

**Critères d'acceptation:**
- [ ] Bouton "Supprimer" (icône poubelle) dans chaque ligne de table
- [ ] Click ouvre dialog de confirmation
- [ ] Dialog affiche nom de cassette et avertissement
- [ ] Confirmation envoie DELETE /api/cassettes/:name
- [ ] Cassette disparaît de la liste après suppression
- [ ] Toast de confirmation affiché

### US-3.0.3: Mettre à Jour Métadonnées
**En tant qu'** utilisateur
**Je veux** pouvoir modifier la description d'une cassette
**Afin de** maintenir des métadonnées à jour

**Critères d'acceptation:**
- [ ] Bouton "Éditer" (icône crayon) dans chaque ligne
- [ ] Click ouvre dialog avec formulaire pré-rempli
- [ ] Champs modifiables: Description
- [ ] Soumission envoie PUT /api/cassettes/:name
- [ ] Liste se rafraîchit après modification
- [ ] Toast de confirmation affiché

---

## 🏗️ Architecture Backend

### Nouveaux Endpoints

#### POST /api/cassettes
**Créer une nouvelle cassette**

```http
POST /api/cassettes HTTP/1.1
Host: localhost:8889
Content-Type: application/json

{
  "name": "my-new-cassette",
  "description": "Test API for user service",
  "mode": "auto"
}
```

**Réponse 201 Created:**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "CassetteResource",
  "@id": "http://localhost:8889/api/cassettes/my-new-cassette",
  "name": "my-new-cassette",
  "description": "Test API for user service",
  "recorded_at": "2025-10-27T10:30:00Z",
  "interactions_count": 0,
  "hydra:link": [
    {
      "@type": "hydra:Link",
      "hydra:target": "http://localhost:8889/api/cassettes/my-new-cassette",
      "title": "View Cassette Details",
      "hydra:operation": [{"@type": "http://schema.org/ViewAction", "method": "GET"}]
    },
    {
      "@type": "hydra:Link",
      "hydra:target": "http://localhost:8889/api/cassettes/my-new-cassette",
      "title": "Delete Cassette",
      "hydra:operation": [{"@type": "http://schema.org/DeleteAction", "method": "DELETE"}]
    }
  ]
}
```

**Erreurs:**
- `400 Bad Request` - Nom invalide (caractères interdits, vide)
- `409 Conflict` - Cassette avec ce nom existe déjà
- `500 Internal Server Error` - Erreur création fichier

#### DELETE /api/cassettes/:name
**Supprimer une cassette existante**

```http
DELETE /api/cassettes/my-new-cassette HTTP/1.1
Host: localhost:8889
```

**Réponse 204 No Content:**
```
(body vide)
```

**Erreurs:**
- `404 Not Found` - Cassette n'existe pas
- `500 Internal Server Error` - Erreur suppression fichier

#### PUT /api/cassettes/:name
**Mettre à jour métadonnées**

```http
PUT /api/cassettes/my-new-cassette HTTP/1.1
Host: localhost:8889
Content-Type: application/json

{
  "description": "Updated description for user service API"
}
```

**Réponse 200 OK:**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "CassetteResource",
  "@id": "http://localhost:8889/api/cassettes/my-new-cassette",
  "name": "my-new-cassette",
  "description": "Updated description for user service API",
  "recorded_at": "2025-10-27T10:30:00Z",
  "interactions_count": 0
}
```

**Erreurs:**
- `404 Not Found` - Cassette n'existe pas
- `400 Bad Request` - Données invalides
- `500 Internal Server Error` - Erreur mise à jour fichier

---

## 💻 Implémentation Backend

### Fichiers à Créer/Modifier

```
src/api/handlers/
├── cassette_create.rs       # Handler POST /api/cassettes
├── cassette_delete.rs       # Handler DELETE /api/cassettes/:name
├── cassette_update.rs       # Handler PUT /api/cassettes/:name
└── mod.rs                   # Export nouveaux handlers

src/api/
├── routes.rs                # Ajouter nouvelles routes
└── validation.rs            # Validation noms cassettes (nouveau)

src/cassette.rs              # Ajouter méthodes create(), delete(), update()
```

### Code Backend - Handler POST

**`src/api/handlers/cassette_create.rs`:**

```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::api::models::cassette_resource::CassetteResource;
use crate::api::ApiState;
use crate::cassette::Cassette;

#[derive(Debug, Deserialize)]
pub struct CreateCassetteRequest {
    pub name: String,
    pub description: Option<String>,
    pub mode: Option<String>, // "auto", "record", "replay"
}

#[derive(Debug, Serialize)]
pub struct CreateCassetteResponse {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub recorded_at: String,
    pub interactions_count: usize,
    #[serde(rename = "hydra:link")]
    pub links: Vec<HydraLink>,
}

pub async fn create_cassette(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<CreateCassetteRequest>,
) -> Result<(StatusCode, Json<CreateCassetteResponse>), (StatusCode, String)> {
    // Validation du nom
    if !is_valid_cassette_name(&payload.name) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid cassette name. Use alphanumeric, hyphens, underscores only.".to_string(),
        ));
    }

    // Vérifier si cassette existe déjà
    let cassette_path = state.cassette_dir.join(format!("{}.json", payload.name));
    if cassette_path.exists() {
        return Err((
            StatusCode::CONFLICT,
            format!("Cassette '{}' already exists", payload.name),
        ));
    }

    // Créer cassette vide
    let cassette = Cassette::new(
        payload.name.clone(),
        payload.description.clone().unwrap_or_default(),
    );

    // Sauvegarder sur disque
    cassette.save(&state.cassette_dir)
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create cassette: {}", e),
        ))?;

    // Construire réponse
    let response = CreateCassetteResponse {
        context: "https://www.w3.org/ns/hydra/core".to_string(),
        type_: "CassetteResource".to_string(),
        id: format!("{}/api/cassettes/{}", state.base_url, payload.name),
        name: payload.name.clone(),
        description: payload.description,
        recorded_at: chrono::Utc::now().to_rfc3339(),
        interactions_count: 0,
        links: vec![
            HydraLink {
                type_: "hydra:Link".to_string(),
                target: format!("{}/api/cassettes/{}", state.base_url, payload.name),
                title: "View Cassette Details".to_string(),
                operation: vec![HydraOperation {
                    type_: "http://schema.org/ViewAction".to_string(),
                    method: "GET".to_string(),
                }],
            },
            HydraLink {
                type_: "hydra:Link".to_string(),
                target: format!("{}/api/cassettes/{}", state.base_url, payload.name),
                title: "Delete Cassette".to_string(),
                operation: vec![HydraOperation {
                    type_: "http://schema.org/DeleteAction".to_string(),
                    method: "DELETE".to_string(),
                }],
            },
        ],
    };

    Ok((StatusCode::CREATED, Json(response)))
}

fn is_valid_cassette_name(name: &str) -> bool {
    // Alphanumérique, hyphens, underscores, 1-100 caractères
    let re = regex::Regex::new(r"^[a-zA-Z0-9_-]{1,100}$").unwrap();
    re.is_match(name)
}

#[derive(Debug, Serialize)]
struct HydraLink {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "hydra:target")]
    target: String,
    title: String,
    #[serde(rename = "hydra:operation")]
    operation: Vec<HydraOperation>,
}

#[derive(Debug, Serialize)]
struct HydraOperation {
    #[serde(rename = "@type")]
    type_: String,
    method: String,
}
```

### Code Backend - Handler DELETE

**`src/api/handlers/cassette_delete.rs`:**

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use crate::api::ApiState;

pub async fn delete_cassette(
    State(state): State<Arc<ApiState>>,
    Path(name): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let cassette_path = state.cassette_dir.join(format!("{}.json", name));

    if !cassette_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Cassette '{}' not found", name),
        ));
    }

    std::fs::remove_file(&cassette_path)
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete cassette: {}", e),
        ))?;

    Ok(StatusCode::NO_CONTENT)
}
```

### Routes Modification

**`src/api/routes.rs`:**

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

pub fn create_routes(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/api", get(handlers::root::get_api_root))
        .route("/api/cassettes", get(handlers::cassettes::get_cassettes))
        .route("/api/cassettes", post(handlers::cassette_create::create_cassette)) // NOUVEAU
        .route("/api/cassettes/:name", get(handlers::cassette::get_cassette))
        .route("/api/cassettes/:name", put(handlers::cassette_update::update_cassette)) // NOUVEAU
        .route("/api/cassettes/:name", delete(handlers::cassette_delete::delete_cassette)) // NOUVEAU
        .route("/api/cassettes/:name/interactions", get(handlers::interactions::get_interactions))
        .with_state(state)
}
```

---

## 🅰️ Implémentation Frontend

### Fichiers à Créer

```
frontend/src/app/features/cassettes/components/
├── cassette-create-dialog/
│   ├── cassette-create-dialog.component.ts
│   ├── cassette-create-dialog.component.html
│   ├── cassette-create-dialog.component.scss
│   └── cassette-create-dialog.component.spec.ts
└── cassette-delete-dialog/
    ├── cassette-delete-dialog.component.ts
    ├── cassette-delete-dialog.component.html
    ├── cassette-delete-dialog.component.scss
    └── cassette-delete-dialog.component.spec.ts
```

### Code Frontend - Create Dialog Component

**`cassette-create-dialog.component.ts`:**

```typescript
import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatSelectModule } from '@angular/material/select';

@Component({
  selector: 'app-cassette-create-dialog',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MatDialogModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatSelectModule,
  ],
  templateUrl: './cassette-create-dialog.component.html',
  styleUrls: ['./cassette-create-dialog.component.scss'],
})
export class CassetteCreateDialogComponent {
  private fb = inject(FormBuilder);
  private dialogRef = inject(MatDialogRef<CassetteCreateDialogComponent>);

  form: FormGroup = this.fb.group({
    name: ['', [
      Validators.required,
      Validators.pattern(/^[a-zA-Z0-9_-]{1,100}$/),
    ]],
    description: [''],
    mode: ['auto', Validators.required],
  });

  modes = [
    { value: 'auto', label: 'Auto (record if missing, else replay)' },
    { value: 'record', label: 'Record (always record)' },
    { value: 'replay', label: 'Replay (always replay)' },
  ];

  get nameControl() {
    return this.form.get('name');
  }

  get nameErrors(): string | null {
    if (this.nameControl?.hasError('required')) {
      return 'Name is required';
    }
    if (this.nameControl?.hasError('pattern')) {
      return 'Use alphanumeric, hyphens, underscores only (1-100 chars)';
    }
    return null;
  }

  onSubmit(): void {
    if (this.form.valid) {
      this.dialogRef.close(this.form.value);
    }
  }

  onCancel(): void {
    this.dialogRef.close();
  }
}
```

**`cassette-create-dialog.component.html`:**

```html
<h2 mat-dialog-title>Create New Cassette</h2>

<form [formGroup]="form" (ngSubmit)="onSubmit()">
  <mat-dialog-content>
    <mat-form-field appearance="outline" class="full-width">
      <mat-label>Name</mat-label>
      <input matInput formControlName="name" placeholder="my-api-test" autofocus>
      <mat-error *ngIf="nameErrors">{{ nameErrors }}</mat-error>
      <mat-hint>Alphanumeric, hyphens, underscores only</mat-hint>
    </mat-form-field>

    <mat-form-field appearance="outline" class="full-width">
      <mat-label>Description (optional)</mat-label>
      <textarea
        matInput
        formControlName="description"
        placeholder="Test cassette for user API"
        rows="3"
      ></textarea>
    </mat-form-field>

    <mat-form-field appearance="outline" class="full-width">
      <mat-label>Mode</mat-label>
      <mat-select formControlName="mode">
        <mat-option *ngFor="let mode of modes" [value]="mode.value">
          {{ mode.label }}
        </mat-option>
      </mat-select>
    </mat-form-field>
  </mat-dialog-content>

  <mat-dialog-actions align="end">
    <button mat-button type="button" (click)="onCancel()">Cancel</button>
    <button mat-raised-button color="primary" type="submit" [disabled]="!form.valid">
      Create
    </button>
  </mat-dialog-actions>
</form>
```

### NgRx Actions

**`cassette.actions.ts` (additions):**

```typescript
import { createAction, props } from '@ngrx/store';
import { CreateCassetteRequest, UpdateCassetteRequest } from '../models';

// CREATE
export const createCassette = createAction(
  '[Cassette] Create Cassette',
  props<{ request: CreateCassetteRequest }>()
);

export const createCassetteSuccess = createAction(
  '[Cassette] Create Cassette Success',
  props<{ cassette: Cassette }>()
);

export const createCassetteFailure = createAction(
  '[Cassette] Create Cassette Failure',
  props<{ error: string }>()
);

// DELETE
export const deleteCassette = createAction(
  '[Cassette] Delete Cassette',
  props<{ name: string }>()
);

export const deleteCassetteSuccess = createAction(
  '[Cassette] Delete Cassette Success',
  props<{ name: string }>()
);

export const deleteCassetteFailure = createAction(
  '[Cassette] Delete Cassette Failure',
  props<{ error: string }>()
);

// UPDATE
export const updateCassette = createAction(
  '[Cassette] Update Cassette',
  props<{ name: string; request: UpdateCassetteRequest }>()
);

export const updateCassetteSuccess = createAction(
  '[Cassette] Update Cassette Success',
  props<{ cassette: Cassette }>()
);

export const updateCassetteFailure = createAction(
  '[Cassette] Update Cassette Failure',
  props<{ error: string }>()
);
```

### NgRx Effects

**`cassette.effects.ts` (additions):**

```typescript
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, catchError, switchMap, tap } from 'rxjs/operators';
import { MatSnackBar } from '@angular/material/snack-bar';
import * as CassetteActions from './cassette.actions';
import { AlcaeusService } from '../../../core/services/alcaeus.service';

@Injectable()
export class CassetteEffects {
  private actions$ = inject(Actions);
  private alcaeusService = inject(AlcaeusService);
  private snackBar = inject(MatSnackBar);

  // CREATE
  createCassette$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.createCassette),
      switchMap(({ request }) =>
        this.alcaeusService.createCassette(request).pipe(
          map(cassette => CassetteActions.createCassetteSuccess({ cassette })),
          catchError(error => of(CassetteActions.createCassetteFailure({
            error: error.message || 'Failed to create cassette'
          })))
        )
      )
    )
  );

  createCassetteSuccess$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.createCassetteSuccess),
      tap(({ cassette }) => {
        this.snackBar.open(`Cassette "${cassette.name}" created successfully`, 'Close', {
          duration: 3000,
        });
      })
    ),
    { dispatch: false }
  );

  // DELETE
  deleteCassette$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.deleteCassette),
      switchMap(({ name }) =>
        this.alcaeusService.deleteCassette(name).pipe(
          map(() => CassetteActions.deleteCassetteSuccess({ name })),
          catchError(error => of(CassetteActions.deleteCassetteFailure({
            error: error.message || 'Failed to delete cassette'
          })))
        )
      )
    )
  );

  deleteCassetteSuccess$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.deleteCassetteSuccess),
      tap(({ name }) => {
        this.snackBar.open(`Cassette "${name}" deleted successfully`, 'Close', {
          duration: 3000,
        });
      })
    ),
    { dispatch: false }
  );
}
```

---

## 🧪 Tests

### Tests Backend

**`tests/api/cassette_create_test.rs`:**

```rust
#[tokio::test]
async fn test_create_cassette_success() {
    let app = test_app().await;

    let response = app
        .post("/api/cassettes")
        .json(&json!({
            "name": "test-cassette",
            "description": "Test description",
            "mode": "auto"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let body: CreateCassetteResponse = response.json().await;
    assert_eq!(body.name, "test-cassette");
    assert_eq!(body.interactions_count, 0);
}

#[tokio::test]
async fn test_create_cassette_duplicate() {
    let app = test_app().await;

    // Create first time
    app.post("/api/cassettes")
        .json(&json!({"name": "duplicate", "mode": "auto"}))
        .await;

    // Try to create again
    let response = app
        .post("/api/cassettes")
        .json(&json!({"name": "duplicate", "mode": "auto"}))
        .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_delete_cassette_success() {
    let app = test_app().await;

    // Create cassette
    app.post("/api/cassettes")
        .json(&json!({"name": "to-delete", "mode": "auto"}))
        .await;

    // Delete it
    let response = app.delete("/api/cassettes/to-delete").await;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let response = app.get("/api/cassettes/to-delete").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
```

### Tests Frontend

**`cassette-create-dialog.component.spec.ts`:**

```typescript
describe('CassetteCreateDialogComponent', () => {
  let component: CassetteCreateDialogComponent;
  let fixture: ComponentFixture<CassetteCreateDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CassetteCreateDialogComponent],
      providers: [
        { provide: MatDialogRef, useValue: { close: jest.fn() } },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(CassetteCreateDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should validate name pattern', () => {
    const nameControl = component.form.get('name');

    nameControl?.setValue('invalid name!@#');
    expect(nameControl?.hasError('pattern')).toBeTruthy();

    nameControl?.setValue('valid-name_123');
    expect(nameControl?.hasError('pattern')).toBeFalsy();
  });

  it('should submit valid form', () => {
    const dialogRef = TestBed.inject(MatDialogRef);

    component.form.patchValue({
      name: 'test-cassette',
      description: 'Test',
      mode: 'auto',
    });

    component.onSubmit();

    expect(dialogRef.close).toHaveBeenCalledWith({
      name: 'test-cassette',
      description: 'Test',
      mode: 'auto',
    });
  });
});
```

---

## ✅ Critères d'Achèvement

### Backend
- [ ] POST /api/cassettes fonctionne (201 Created)
- [ ] DELETE /api/cassettes/:name fonctionne (204 No Content)
- [ ] PUT /api/cassettes/:name fonctionne (200 OK)
- [ ] Validation nom cassette fonctionne (400 Bad Request)
- [ ] Gestion duplicates fonctionne (409 Conflict)
- [ ] Tests unitaires passent (10+ tests)

### Frontend
- [ ] Dialog création s'ouvre et se ferme
- [ ] Formulaire valide input temps réel
- [ ] Soumission envoie POST et affiche toast
- [ ] Dialog confirmation suppression fonctionne
- [ ] Suppression envoie DELETE et met à jour liste
- [ ] Tests unitaires passent (15+ tests)

### Intégration
- [ ] Créer cassette via UI → apparaît dans liste
- [ ] Supprimer cassette via UI → disparaît de liste
- [ ] Modifier description via UI → change dans détail
- [ ] Tests E2E passent (5+ scénarios)

---

## 📚 Ressources

- **Hydra Operations**: https://www.hydra-cg.com/spec/latest/core/#supported-operations
- **Angular Material Dialog**: https://material.angular.io/components/dialog/overview
- **Angular Reactive Forms**: https://angular.io/guide/reactive-forms
- **Axum Routing**: https://docs.rs/axum/latest/axum/routing/

---

**Auteur:** Claude Code + Équipe Magnéto-Serge
**Date:** 2025-10-26
