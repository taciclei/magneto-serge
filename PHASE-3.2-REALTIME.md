# Phase 3.2: Mises à Jour en Temps Réel (WebSocket)

**Date:** À définir
**Durée estimée:** 5-7 jours
**Statut:** 📋 Planification
**Priorité:** 🟢 Basse (Feature optionnelle mais impressionnante)

---

## 📋 Objectif

Implémenter un système de notifications en temps réel via WebSocket pour informer les utilisateurs des changements de cassettes sans rafraîchir manuellement.

**Cas d'usage:**
- User A crée une cassette → User B voit apparaître automatiquement
- User A supprime une cassette → User B la voit disparaître
- Multi-fenêtre: créer dans tab 1 → apparaît dans tab 2

---

## 🎯 User Stories

### US-3.2.1: Connexion WebSocket Automatique
**En tant qu'** utilisateur
**Je veux** que l'application se connecte automatiquement au WebSocket
**Afin de** recevoir les mises à jour en temps réel

**Critères d'acceptation:**
- [ ] WebSocket se connecte automatiquement au chargement de l'application
- [ ] Badge "Live" affiché quand connecté
- [ ] Badge "Reconnecting..." affiché lors de reconnexion
- [ ] Reconnexion automatique avec backoff exponentiel

### US-3.2.2: Notifications de Création
**En tant qu'** utilisateur
**Je veux** être notifié quand une nouvelle cassette est créée
**Afin de** voir la liste se mettre à jour automatiquement

**Critères d'acceptation:**
- [ ] Nouvelle cassette apparaît dans liste sans rafraîchir
- [ ] Snackbar affiche "Nouvelle cassette: {name}"
- [ ] Animation d'apparition (fade-in)

### US-3.2.3: Notifications de Suppression
**En tant qu'** utilisateur
**Je veux** être notifié quand une cassette est supprimée
**Afin de** voir la liste se mettre à jour automatiquement

**Critères d'acceptation:**
- [ ] Cassette disparaît de la liste sans rafraîchir
- [ ] Snackbar affiche "Cassette supprimée: {name}"
- [ ] Animation de disparition (fade-out)

---

## 🏗️ Architecture Backend

### WebSocket Endpoint

#### GET /ws
**Connexion WebSocket**

```
ws://localhost:8889/ws
```

**Protocole:**
- Client → Server: Ping toutes les 30s
- Server → Client: Pong
- Server → Client: Événements JSON

**Messages Server → Client:**

```json
// CassetteCreated
{
  "type": "CassetteCreated",
  "cassette": {
    "name": "new-cassette",
    "description": "Test",
    "recorded_at": "2025-10-27T10:00:00Z",
    "interactions_count": 0
  },
  "timestamp": "2025-10-27T10:00:00Z"
}

// CassetteDeleted
{
  "type": "CassetteDeleted",
  "name": "old-cassette",
  "timestamp": "2025-10-27T10:01:00Z"
}

// CassetteUpdated
{
  "type": "CassetteUpdated",
  "cassette": {
    "name": "updated-cassette",
    "description": "New description",
    "recorded_at": "2025-10-27T09:00:00Z",
    "interactions_count": 5
  },
  "timestamp": "2025-10-27T10:02:00Z"
}
```

---

## 💻 Implémentation Backend

### Fichiers à Créer

```
src/api/
├── websocket.rs              # WebSocket handler principal
├── events.rs                 # Définition des événements
└── broadcast.rs              # Système de broadcast aux clients

examples/
└── hydra_api_server.rs       # Modifier pour ajouter route WebSocket
```

### Code Backend - WebSocket Handler

**`src/api/websocket.rs`:**

```rust
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::api::{ApiState, events::CassetteEvent};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ApiState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ApiState>) {
    let (mut sender, mut receiver) = socket.split();

    // S'abonner au broadcast channel
    let mut event_rx = state.event_tx.subscribe();

    // Envoyer ping toutes les 30s
    let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(30));

    loop {
        tokio::select! {
            // Recevoir événement du broadcast
            Ok(event) = event_rx.recv() => {
                let json = serde_json::to_string(&event).unwrap();
                if sender.send(Message::Text(json)).await.is_err() {
                    break; // Client déconnecté
                }
            }

            // Envoyer ping
            _ = ping_interval.tick() => {
                if sender.send(Message::Ping(vec![])).await.is_err() {
                    break; // Client déconnecté
                }
            }

            // Recevoir message du client
            Some(Ok(msg)) = receiver.next() => {
                match msg {
                    Message::Pong(_) => {
                        // Client répond au ping, connexion active
                    }
                    Message::Close(_) => {
                        break; // Client ferme la connexion
                    }
                    _ => {}
                }
            }

            else => break,
        }
    }

    tracing::info!("WebSocket client disconnected");
}
```

**`src/api/events.rs`:**

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::cassette::Cassette;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CassetteEvent {
    CassetteCreated {
        cassette: Cassette,
        timestamp: DateTime<Utc>,
    },
    CassetteDeleted {
        name: String,
        timestamp: DateTime<Utc>,
    },
    CassetteUpdated {
        cassette: Cassette,
        timestamp: DateTime<Utc>,
    },
}

impl CassetteEvent {
    pub fn created(cassette: Cassette) -> Self {
        Self::CassetteCreated {
            cassette,
            timestamp: Utc::now(),
        }
    }

    pub fn deleted(name: String) -> Self {
        Self::CassetteDeleted {
            name,
            timestamp: Utc::now(),
        }
    }

    pub fn updated(cassette: Cassette) -> Self {
        Self::CassetteUpdated {
            cassette,
            timestamp: Utc::now(),
        }
    }
}
```

**`src/api/mod.rs` (modifications):**

```rust
use tokio::sync::broadcast;
use crate::api::events::CassetteEvent;

pub struct ApiState {
    pub cassette_dir: PathBuf,
    pub base_url: String,
    pub event_tx: broadcast::Sender<CassetteEvent>, // NOUVEAU
}

impl ApiState {
    pub fn new(cassette_dir: PathBuf, base_url: String) -> Arc<Self> {
        let (event_tx, _) = broadcast::channel(100);
        Arc::new(Self {
            cassette_dir,
            base_url,
            event_tx,
        })
    }

    // Broadcast un événement à tous les clients WebSocket
    pub fn broadcast_event(&self, event: CassetteEvent) {
        let _ = self.event_tx.send(event);
    }
}
```

**Modifier handlers pour broadcaster:**

```rust
// Dans cassette_create.rs
pub async fn create_cassette(...) -> ... {
    // ... créer cassette ...

    // Broadcaster l'événement
    state.broadcast_event(CassetteEvent::created(cassette.clone()));

    Ok((StatusCode::CREATED, Json(response)))
}

// Dans cassette_delete.rs
pub async fn delete_cassette(...) -> ... {
    // ... supprimer cassette ...

    // Broadcaster l'événement
    state.broadcast_event(CassetteEvent::deleted(name.clone()));

    Ok(StatusCode::NO_CONTENT)
}
```

---

## 🅰️ Implémentation Frontend

### Fichiers à Créer

```
frontend/src/app/core/services/
└── websocket.service.ts      # Service WebSocket

frontend/src/app/features/cassettes/store/
└── websocket.effects.ts      # Effets NgRx pour WebSocket

frontend/src/app/core/components/
└── connection-status/        # Badge "Live" / "Reconnecting..."
    ├── connection-status.component.ts
    ├── connection-status.component.html
    └── connection-status.component.scss
```

### Code Frontend - WebSocket Service

**`websocket.service.ts`:**

```typescript
import { Injectable, inject } from '@angular/core';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { Observable, timer, Subject, EMPTY } from 'rxjs';
import { retryWhen, tap, delayWhen, catchError } from 'rxjs/operators';
import { environment } from '../../../environments/environment';

export interface CassetteEvent {
  type: 'CassetteCreated' | 'CassetteDeleted' | 'CassetteUpdated';
  cassette?: any;
  name?: string;
  timestamp: string;
}

@Injectable({
  providedIn: 'root'
})
export class WebSocketService {
  private socket$: WebSocketSubject<CassetteEvent> | null = null;
  private messagesSubject$ = new Subject<CassetteEvent>();
  private reconnectAttempt = 0;

  public messages$ = this.messagesSubject$.asObservable();

  connect(): void {
    if (!this.socket$ || this.socket$.closed) {
      const wsUrl = environment.apiUrl.replace('http', 'ws').replace('/api', '/ws');

      this.socket$ = webSocket<CassetteEvent>({
        url: wsUrl,
        openObserver: {
          next: () => {
            console.log('WebSocket connected');
            this.reconnectAttempt = 0;
          }
        },
        closeObserver: {
          next: () => {
            console.log('WebSocket closed');
            this.reconnect();
          }
        },
      });

      this.socket$
        .pipe(
          tap(message => this.messagesSubject$.next(message)),
          retryWhen(errors =>
            errors.pipe(
              tap(err => console.error('WebSocket error:', err)),
              delayWhen(() => this.getReconnectDelay())
            )
          ),
          catchError(err => {
            console.error('WebSocket fatal error:', err);
            return EMPTY;
          })
        )
        .subscribe();
    }
  }

  private reconnect(): void {
    this.reconnectAttempt++;
    timer(this.getReconnectDelayMs()).subscribe(() => {
      console.log(`Reconnecting (attempt ${this.reconnectAttempt})...`);
      this.connect();
    });
  }

  private getReconnectDelay(): Observable<number> {
    return timer(this.getReconnectDelayMs());
  }

  private getReconnectDelayMs(): number {
    // Backoff exponentiel: 1s, 2s, 4s, 8s, max 30s
    const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempt), 30000);
    return delay;
  }

  disconnect(): void {
    if (this.socket$) {
      this.socket$.complete();
      this.socket$ = null;
    }
  }
}
```

### NgRx WebSocket Effects

**`websocket.effects.ts`:**

```typescript
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { tap, map } from 'rxjs/operators';
import { MatSnackBar } from '@angular/material/snack-bar';
import { WebSocketService } from '../../../core/services/websocket.service';
import * as CassetteActions from './cassette.actions';

@Injectable()
export class WebSocketEffects {
  private actions$ = inject(Actions);
  private wsService = inject(WebSocketService);
  private snackBar = inject(MatSnackBar);

  // Connecter WebSocket au démarrage
  init$ = createEffect(() =>
    this.actions$.pipe(
      ofType('@ngrx/effects/init'),
      tap(() => this.wsService.connect())
    ),
    { dispatch: false }
  );

  // Écouter les événements WebSocket
  listenToWebSocket$ = createEffect(() =>
    this.wsService.messages$.pipe(
      map(event => {
        switch (event.type) {
          case 'CassetteCreated':
            this.showNotification(`New cassette: ${event.cassette.name}`);
            return CassetteActions.cassetteCreaatedViaWebSocket({ cassette: event.cassette });

          case 'CassetteDeleted':
            this.showNotification(`Cassette deleted: ${event.name}`);
            return CassetteActions.cassetteDeletedViaWebSocket({ name: event.name! });

          case 'CassetteUpdated':
            this.showNotification(`Cassette updated: ${event.cassette.name}`);
            return CassetteActions.cassetteUpdatedViaWebSocket({ cassette: event.cassette });

          default:
            return { type: 'UNKNOWN_EVENT' };
        }
      })
    )
  );

  private showNotification(message: string): void {
    this.snackBar.open(message, 'Close', {
      duration: 3000,
      horizontalPosition: 'end',
      verticalPosition: 'top',
    });
  }
}
```

### NgRx Actions (additions)

**`cassette.actions.ts`:**

```typescript
// WebSocket events
export const cassetteCreaatedViaWebSocket = createAction(
  '[WebSocket] Cassette Created',
  props<{ cassette: Cassette }>()
);

export const cassetteDeletedViaWebSocket = createAction(
  '[WebSocket] Cassette Deleted',
  props<{ name: string }>()
);

export const cassetteUpdatedViaWebSocket = createAction(
  '[WebSocket] Cassette Updated',
  props<{ cassette: Cassette }>()
);
```

### NgRx Reducer (modifications)

**`cassette.reducer.ts`:**

```typescript
on(CassetteActions.cassetteCreaatedViaWebSocket, (state, { cassette }) => ({
  ...state,
  cassettes: [...state.cassettes, cassette],
  totalItems: state.totalItems + 1,
})),

on(CassetteActions.cassetteDeletedViaWebSocket, (state, { name }) => ({
  ...state,
  cassettes: state.cassettes.filter(c => c.name !== name),
  totalItems: state.totalItems - 1,
})),

on(CassetteActions.cassetteUpdatedViaWebSocket, (state, { cassette }) => ({
  ...state,
  cassettes: state.cassettes.map(c =>
    c.name === cassette.name ? cassette : c
  ),
}))
```

---

## 🧪 Tests

### Tests Backend

```rust
#[tokio::test]
async fn test_websocket_connection() {
    let app = test_app().await;

    let mut ws = app.websocket("/ws").await;

    // Attendre message ping
    let msg = ws.recv().await.unwrap();
    assert!(matches!(msg, Message::Ping(_)));
}

#[tokio::test]
async fn test_websocket_broadcast_create() {
    let app = test_app().await;

    let mut ws = app.websocket("/ws").await;

    // Créer cassette via API
    app.post("/api/cassettes")
        .json(&json!({"name": "test", "mode": "auto"}))
        .await;

    // Recevoir événement WebSocket
    let msg = ws.recv_text().await.unwrap();
    let event: CassetteEvent = serde_json::from_str(&msg).unwrap();

    assert!(matches!(event, CassetteEvent::CassetteCreated { .. }));
}
```

### Tests Frontend

```typescript
describe('WebSocketService', () => {
  it('should connect to WebSocket', () => {
    service.connect();
    // Vérifier que la connexion est établie
    expect(service['socket$']).toBeTruthy();
  });

  it('should emit messages', (done) => {
    service.messages$.subscribe(event => {
      expect(event.type).toBe('CassetteCreated');
      done();
    });

    service.connect();
    // Simuler message entrant
    service['messagesSubject$'].next({
      type: 'CassetteCreated',
      cassette: { name: 'test' },
      timestamp: new Date().toISOString(),
    });
  });
});
```

---

## ✅ Critères d'Achèvement

### Backend
- [ ] WebSocket endpoint /ws fonctionne
- [ ] Ping/Pong keep-alive toutes les 30s
- [ ] Broadcast CassetteCreated fonctionne
- [ ] Broadcast CassetteDeleted fonctionne
- [ ] Support 100+ clients simultanés
- [ ] Tests WebSocket passent (5+ tests)

### Frontend
- [ ] WebSocket se connecte automatiquement
- [ ] Reconnexion automatique avec backoff exponentiel
- [ ] Badge "Live" affiché quand connecté
- [ ] Événements WebSocket mettent à jour liste
- [ ] Notifications snackbar affichées
- [ ] Tests unitaires passent (5+ tests)

### Intégration
- [ ] Créer cassette dans tab 1 → apparaît dans tab 2
- [ ] Supprimer cassette dans tab 1 → disparaît dans tab 2
- [ ] Tests E2E multi-fenêtre passent

---

**Auteur:** Claude Code + Équipe Magnéto-Serge
**Date:** 2025-10-26
