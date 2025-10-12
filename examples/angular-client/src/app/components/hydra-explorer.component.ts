/**
 * Composant explorateur Hydra
 *
 * Démontre la navigation Hydra dynamique:
 * - Découverte automatique de l'API
 * - Navigation par liens Hydra
 * - Exécution d'opérations
 * - Affichage des ressources JSON-LD
 */

import { Component, OnInit, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Subject, takeUntil } from 'rxjs';
import { IResource } from 'alcaeus';

import { MagnetoApiService } from '../services/magneto-api.service';
import { HydraClientService } from '../services/hydra-client.service';
import {
  ProxyStatus,
  CassetteInfo,
  StartProxyRequest,
  HydraLink,
  NavigationEventPayload
} from '../models/hydra.models';

@Component({
  selector: 'app-hydra-explorer',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="hydra-explorer">
      <header class="header">
        <h1>🌐 Magneto-Serge Hydra Explorer</h1>
        <div class="config">
          <input
            type="text"
            [(ngModel)]="apiBaseUrl"
            placeholder="API Base URL"
            (change)="onBaseUrlChange()"
          />
          <input
            type="password"
            [(ngModel)]="apiKey"
            placeholder="API Key (optional)"
            (change)="onApiKeyChange()"
          />
        </div>
      </header>

      <!-- Découverte de l'API -->
      <section class="api-discovery" *ngIf="apiInfo">
        <h2>📡 API Discovery</h2>
        <div class="api-info">
          <p><strong>Title:</strong> {{ apiInfo.title }}</p>
          <p><strong>Version:</strong> {{ apiInfo.version }}</p>
          <p><strong>Description:</strong> {{ apiInfo.description }}</p>
        </div>

        <div class="available-actions">
          <h3>Available Actions (Hydra Links):</h3>
          <div class="action-grid">
            <button
              *ngFor="let action of availableActions"
              (click)="performAction(action)"
              class="action-btn"
            >
              {{ action }}
            </button>
          </div>
        </div>
      </section>

      <!-- Navigation Breadcrumb -->
      <section class="breadcrumb" *ngIf="navigationPath.length > 0">
        <h3>🧭 Navigation Path:</h3>
        <div class="path">
          <span
            *ngFor="let step of navigationPath; let i = index"
            (click)="goToStep(i)"
            class="path-step"
          >
            {{ step.title }}
            <span *ngIf="i < navigationPath.length - 1" class="separator">→</span>
          </span>
        </div>
      </section>

      <!-- Statut du Proxy -->
      <section class="proxy-status" *ngIf="proxyStatus">
        <h2>📊 Proxy Status</h2>
        <div class="status-card" [class.running]="proxyStatus.running">
          <div class="status-grid">
            <div class="status-item">
              <span class="label">Running:</span>
              <span class="value" [class.active]="proxyStatus.running">
                {{ proxyStatus.running ? '✓ Yes' : '✗ No' }}
              </span>
            </div>
            <div class="status-item">
              <span class="label">Mode:</span>
              <span class="value">{{ proxyStatus.mode }}</span>
            </div>
            <div class="status-item">
              <span class="label">Port:</span>
              <span class="value">{{ proxyStatus.port }}</span>
            </div>
            <div class="status-item">
              <span class="label">Cassette:</span>
              <span class="value">{{ proxyStatus.cassette || 'None' }}</span>
            </div>
            <div class="status-item">
              <span class="label">Interactions:</span>
              <span class="value">{{ proxyStatus.interactions_count }}</span>
            </div>
            <div class="status-item">
              <span class="label">Uptime:</span>
              <span class="value">{{ proxyStatus.uptime_seconds }}s</span>
            </div>
          </div>

          <!-- Actions disponibles depuis le statut -->
          <div class="next-actions" *ngIf="statusNextActions.length > 0">
            <h4>Next Actions (Hydra):</h4>
            <button
              *ngFor="let action of statusNextActions"
              (click)="followStatusLink(action)"
              class="next-action-btn"
            >
              {{ action }}
            </button>
          </div>
        </div>
      </section>

      <!-- Contrôle du Proxy -->
      <section class="proxy-control">
        <h2>⚙️ Proxy Control</h2>
        <div class="control-form">
          <div class="form-group">
            <label>Cassette Name:</label>
            <input
              type="text"
              [(ngModel)]="startRequest.cassette_name"
              placeholder="test-cassette"
            />
          </div>
          <div class="form-group">
            <label>Mode:</label>
            <select [(ngModel)]="startRequest.mode">
              <option value="auto">Auto</option>
              <option value="record">Record</option>
              <option value="replay">Replay</option>
              <option value="passthrough">Passthrough</option>
            </select>
          </div>
          <div class="form-group">
            <label>Port:</label>
            <input
              type="number"
              [(ngModel)]="startRequest.port"
              placeholder="8888"
            />
          </div>
          <div class="form-group">
            <label>
              <input type="checkbox" [(ngModel)]="startRequest.strict" />
              Strict Mode
            </label>
          </div>
          <div class="button-group">
            <button
              (click)="startProxy()"
              [disabled]="loading || (proxyStatus && proxyStatus.running)"
              class="btn-primary"
            >
              🟢 Start Proxy
            </button>
            <button
              (click)="stopProxy()"
              [disabled]="loading || !proxyStatus || !proxyStatus.running"
              class="btn-danger"
            >
              ⏹️ Stop Proxy
            </button>
            <button (click)="refreshStatus()" [disabled]="loading" class="btn-secondary">
              🔄 Refresh
            </button>
          </div>
        </div>
      </section>

      <!-- Liste des Cassettes -->
      <section class="cassettes-list" *ngIf="cassettes.length > 0">
        <h2>📼 Cassettes ({{ totalCassettes }} total)</h2>
        <div class="cassette-grid">
          <div *ngFor="let cassette of cassettes" class="cassette-card">
            <div class="cassette-header">
              <h3>{{ cassette.name }}</h3>
              <span class="cassette-format">{{ cassette.format }}</span>
            </div>
            <div class="cassette-details">
              <p>
                <strong>Size:</strong> {{ (cassette.size_bytes / 1024).toFixed(1) }} KB
              </p>
              <p><strong>Interactions:</strong> {{ cassette.interactions }}</p>
              <p><strong>Created:</strong> {{ cassette.created_at | date: 'short' }}</p>
            </div>
            <div class="cassette-actions">
              <button (click)="viewCassette(cassette.name)" class="btn-small">
                👁️ View
              </button>
              <button
                (click)="deleteCassette(cassette.name)"
                class="btn-small btn-danger"
              >
                🗑️ Delete
              </button>
            </div>
          </div>
        </div>
      </section>

      <!-- Événements de Navigation -->
      <section class="navigation-events">
        <h2>📡 Navigation Events (Hydra)</h2>
        <div class="events-log">
          <div
            *ngFor="let event of navigationEvents.slice().reverse().slice(0, 10)"
            class="event-item"
            [class]="'event-' + event.type"
          >
            <span class="event-time">{{ event.timestamp | date: 'HH:mm:ss' }}</span>
            <span class="event-type">{{ event.type }}</span>
            <span class="event-details" *ngIf="event.link">
              → {{ event.link.title }}
            </span>
            <span class="event-details" *ngIf="event.error">
              ❌ {{ event.error.message }}
            </span>
          </div>
        </div>
      </section>

      <!-- Ressource JSON-LD Brute -->
      <section class="raw-resource" *ngIf="currentResource">
        <h2>🔍 Current Resource (JSON-LD)</h2>
        <button (click)="showRawJson = !showRawJson" class="toggle-btn">
          {{ showRawJson ? 'Hide' : 'Show' }} Raw JSON
        </button>
        <pre *ngIf="showRawJson" class="json-display">{{ currentResourceJson }}</pre>
      </section>

      <!-- Loading & Errors -->
      <div *ngIf="loading" class="loading-overlay">
        <div class="spinner"></div>
        <p>Loading...</p>
      </div>

      <div *ngIf="error" class="error-message">
        <strong>❌ Error:</strong> {{ error }}
        <button (click)="error = null" class="close-btn">✕</button>
      </div>
    </div>
  `,
  styles: [`
    .hydra-explorer {
      max-width: 1400px;
      margin: 0 auto;
      padding: 20px;
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }

    .header {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      padding: 30px;
      border-radius: 10px;
      margin-bottom: 20px;
    }

    .header h1 {
      margin: 0 0 15px 0;
    }

    .config {
      display: flex;
      gap: 10px;
    }

    .config input {
      flex: 1;
      padding: 10px;
      border: none;
      border-radius: 5px;
      font-size: 14px;
    }

    section {
      background: white;
      padding: 20px;
      margin-bottom: 20px;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    h2 {
      margin-top: 0;
      color: #333;
      border-bottom: 2px solid #667eea;
      padding-bottom: 10px;
    }

    .action-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
      gap: 10px;
      margin-top: 15px;
    }

    .action-btn {
      padding: 12px;
      background: #667eea;
      color: white;
      border: none;
      border-radius: 6px;
      cursor: pointer;
      transition: all 0.3s;
      font-weight: 500;
    }

    .action-btn:hover {
      background: #764ba2;
      transform: translateY(-2px);
      box-shadow: 0 4px 8px rgba(0,0,0,0.2);
    }

    .breadcrumb .path {
      display: flex;
      align-items: center;
      flex-wrap: wrap;
      gap: 10px;
      margin-top: 10px;
    }

    .path-step {
      cursor: pointer;
      color: #667eea;
      font-weight: 500;
      transition: color 0.3s;
    }

    .path-step:hover {
      color: #764ba2;
      text-decoration: underline;
    }

    .separator {
      color: #999;
      margin: 0 5px;
    }

    .status-card {
      border: 3px solid #ddd;
      border-radius: 8px;
      padding: 20px;
      transition: border-color 0.3s;
    }

    .status-card.running {
      border-color: #52c41a;
      background: #f6ffed;
    }

    .status-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 15px;
      margin-bottom: 20px;
    }

    .status-item {
      display: flex;
      flex-direction: column;
      gap: 5px;
    }

    .status-item .label {
      font-size: 12px;
      color: #888;
      text-transform: uppercase;
      font-weight: 600;
    }

    .status-item .value {
      font-size: 18px;
      font-weight: 600;
      color: #333;
    }

    .status-item .value.active {
      color: #52c41a;
    }

    .next-actions {
      border-top: 1px solid #ddd;
      padding-top: 15px;
    }

    .next-action-btn {
      padding: 8px 16px;
      margin: 5px;
      background: #13c2c2;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      transition: all 0.3s;
    }

    .next-action-btn:hover {
      background: #08979c;
    }

    .control-form {
      display: grid;
      gap: 15px;
    }

    .form-group {
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    .form-group label {
      font-weight: 600;
      color: #333;
    }

    .form-group input,
    .form-group select {
      padding: 10px;
      border: 2px solid #ddd;
      border-radius: 6px;
      font-size: 14px;
      transition: border-color 0.3s;
    }

    .form-group input:focus,
    .form-group select:focus {
      outline: none;
      border-color: #667eea;
    }

    .button-group {
      display: flex;
      gap: 10px;
      margin-top: 15px;
    }

    .btn-primary,
    .btn-danger,
    .btn-secondary {
      padding: 12px 24px;
      border: none;
      border-radius: 6px;
      font-size: 14px;
      font-weight: 600;
      cursor: pointer;
      transition: all 0.3s;
    }

    .btn-primary {
      background: #52c41a;
      color: white;
    }

    .btn-primary:hover:not(:disabled) {
      background: #389e0d;
    }

    .btn-danger {
      background: #ff4d4f;
      color: white;
    }

    .btn-danger:hover:not(:disabled) {
      background: #cf1322;
    }

    .btn-secondary {
      background: #1890ff;
      color: white;
    }

    .btn-secondary:hover:not(:disabled) {
      background: #096dd9;
    }

    button:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    .cassette-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: 15px;
      margin-top: 15px;
    }

    .cassette-card {
      border: 2px solid #f0f0f0;
      border-radius: 8px;
      padding: 15px;
      transition: all 0.3s;
    }

    .cassette-card:hover {
      border-color: #667eea;
      box-shadow: 0 4px 12px rgba(102, 126, 234, 0.15);
    }

    .cassette-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 10px;
    }

    .cassette-header h3 {
      margin: 0;
      font-size: 16px;
      color: #333;
    }

    .cassette-format {
      padding: 4px 8px;
      background: #f0f0f0;
      border-radius: 4px;
      font-size: 12px;
      font-weight: 600;
    }

    .cassette-details p {
      margin: 5px 0;
      font-size: 13px;
      color: #666;
    }

    .cassette-actions {
      display: flex;
      gap: 8px;
      margin-top: 12px;
    }

    .btn-small {
      flex: 1;
      padding: 8px;
      background: #f0f0f0;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      font-size: 12px;
      transition: all 0.3s;
    }

    .btn-small:hover {
      background: #d9d9d9;
    }

    .btn-small.btn-danger {
      background: #fff1f0;
      color: #ff4d4f;
    }

    .btn-small.btn-danger:hover {
      background: #ffccc7;
    }

    .events-log {
      max-height: 300px;
      overflow-y: auto;
      border: 1px solid #f0f0f0;
      border-radius: 6px;
      padding: 10px;
    }

    .event-item {
      padding: 8px;
      margin-bottom: 5px;
      border-radius: 4px;
      display: flex;
      gap: 10px;
      font-size: 13px;
    }

    .event-resource\:loaded {
      background: #e6f7ff;
    }

    .event-link\:followed {
      background: #f6ffed;
    }

    .event-operation\:executed {
      background: #fff7e6;
    }

    .event-error {
      background: #fff1f0;
    }

    .event-time {
      color: #888;
      font-weight: 600;
    }

    .event-type {
      color: #667eea;
      font-weight: 600;
    }

    .json-display {
      background: #f5f5f5;
      padding: 15px;
      border-radius: 6px;
      overflow-x: auto;
      font-size: 12px;
      font-family: 'Courier New', monospace;
    }

    .toggle-btn {
      padding: 8px 16px;
      background: #f0f0f0;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      margin-bottom: 10px;
    }

    .loading-overlay {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0, 0, 0, 0.7);
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      color: white;
      z-index: 1000;
    }

    .spinner {
      border: 4px solid rgba(255, 255, 255, 0.3);
      border-top: 4px solid white;
      border-radius: 50%;
      width: 50px;
      height: 50px;
      animation: spin 1s linear infinite;
    }

    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }

    .error-message {
      position: fixed;
      top: 20px;
      right: 20px;
      background: #ff4d4f;
      color: white;
      padding: 15px 20px;
      border-radius: 8px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
      z-index: 1000;
      max-width: 400px;
    }

    .close-btn {
      background: none;
      border: none;
      color: white;
      font-size: 20px;
      cursor: pointer;
      float: right;
      margin-left: 10px;
    }
  `]
})
export class HydraExplorerComponent implements OnInit, OnDestroy {
  private destroy$ = new Subject<void>();

  // Configuration
  apiBaseUrl = 'http://localhost:8889';
  apiKey = '';

  // État de l'API
  apiInfo: any = null;
  availableActions: string[] = [];
  currentResource: IResource | null = null;
  currentResourceJson = '';

  // Navigation
  navigationPath: Array<{ title: string; resource: IResource }> = [];
  navigationEvents: NavigationEventPayload[] = [];

  // Proxy
  proxyStatus: ProxyStatus | null = null;
  statusNextActions: string[] = [];
  startRequest: StartProxyRequest = {
    mode: 'auto',
    cassette_name: 'test-cassette',
    port: 8888,
    strict: false
  };

  // Cassettes
  cassettes: CassetteInfo[] = [];
  totalCassettes = 0;

  // UI
  loading = false;
  error: string | null = null;
  showRawJson = false;

  constructor(
    private magnetoApi: MagnetoApiService,
    private hydraClient: HydraClientService
  ) {}

  ngOnInit(): void {
    // Configurer l'URL de base
    this.magnetoApi.setBaseUrl(this.apiBaseUrl);

    // S'abonner aux événements de navigation Hydra
    this.hydraClient.navigationEvents
      .pipe(takeUntil(this.destroy$))
      .subscribe(event => {
        this.navigationEvents.push(event);
      });

    // Découvrir l'API au démarrage
    this.discoverApi();
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }

  /**
   * Découvre l'API et charge les informations initiales
   */
  discoverApi(): void {
    this.loading = true;
    this.magnetoApi.discoverApi()
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: ({ info, availableActions, resource }) => {
          this.apiInfo = info;
          this.availableActions = availableActions;
          this.currentResource = resource;
          this.updateCurrentResourceJson();
          this.loading = false;

          // Charger le statut et les cassettes
          this.refreshStatus();
          this.loadCassettes();
        },
        error: (err) => {
          this.error = `Failed to discover API: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Exécute une action depuis la liste découverte
   */
  performAction(actionTitle: string): void {
    if (!this.currentResource) return;

    this.loading = true;
    this.magnetoApi.followLink(this.currentResource, actionTitle)
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: (resource) => {
          if (resource) {
            this.navigationPath.push({
              title: actionTitle,
              resource
            });
            this.currentResource = resource;
            this.updateCurrentResourceJson();
          }
          this.loading = false;
        },
        error: (err) => {
          this.error = `Navigation failed: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Navigue vers une étape précédente du breadcrumb
   */
  goToStep(index: number): void {
    if (index < this.navigationPath.length) {
      const step = this.navigationPath[index];
      this.currentResource = step.resource;
      this.navigationPath = this.navigationPath.slice(0, index + 1);
      this.updateCurrentResourceJson();
    }
  }

  /**
   * Rafraîchit le statut du proxy
   */
  refreshStatus(): void {
    this.loading = true;
    this.magnetoApi.getProxyStatus()
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: ({ status, availableActions, resource }) => {
          this.proxyStatus = status;
          this.statusNextActions = availableActions;
          this.loading = false;
        },
        error: (err) => {
          this.error = `Failed to load status: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Suit un lien depuis le statut
   */
  followStatusLink(linkTitle: string): void {
    // Implementation similaire à performAction
    console.log('Following link:', linkTitle);
  }

  /**
   * Démarre le proxy
   */
  startProxy(): void {
    this.loading = true;
    this.magnetoApi.startProxy(this.startRequest)
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: (result: any) => {
          this.loading = false;
          this.refreshStatus();
        },
        error: (err) => {
          this.error = `Failed to start proxy: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Arrête le proxy
   */
  stopProxy(): void {
    this.loading = true;
    this.magnetoApi.stopProxy()
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: (result: any) => {
          this.loading = false;
          this.refreshStatus();
        },
        error: (err) => {
          this.error = `Failed to stop proxy: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Charge la liste des cassettes
   */
  loadCassettes(): void {
    this.magnetoApi.listCassettes()
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: ({ cassettes, totalItems }) => {
          this.cassettes = cassettes;
          this.totalCassettes = totalItems;
        },
        error: (err) => {
          console.error('Failed to load cassettes:', err);
        }
      });
  }

  /**
   * Affiche une cassette
   */
  viewCassette(name: string): void {
    this.loading = true;
    this.magnetoApi.getCassette(name)
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: ({ cassette, resource }) => {
          this.currentResource = resource;
          this.updateCurrentResourceJson();
          this.loading = false;
        },
        error: (err) => {
          this.error = `Failed to load cassette: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Supprime une cassette
   */
  deleteCassette(name: string): void {
    if (!confirm(`Delete cassette "${name}"?`)) return;

    this.loading = true;
    this.magnetoApi.deleteCassette(name)
      .pipe(takeUntil(this.destroy$))
      .subscribe({
        next: () => {
          this.loadCassettes();
          this.loading = false;
        },
        error: (err) => {
          this.error = `Failed to delete cassette: ${err.message}`;
          this.loading = false;
        }
      });
  }

  /**
   * Gestionnaire de changement d'URL de base
   */
  onBaseUrlChange(): void {
    this.magnetoApi.setBaseUrl(this.apiBaseUrl);
    this.hydraClient.clearCache();
    this.discoverApi();
  }

  /**
   * Gestionnaire de changement de clé API
   */
  onApiKeyChange(): void {
    if (this.apiKey) {
      this.magnetoApi.setApiKey(this.apiKey);
    }
  }

  /**
   * Met à jour l'affichage JSON de la ressource courante
   */
  private updateCurrentResourceJson(): void {
    if (this.currentResource) {
      this.currentResourceJson = JSON.stringify(this.currentResource, null, 2);
    }
  }
}
