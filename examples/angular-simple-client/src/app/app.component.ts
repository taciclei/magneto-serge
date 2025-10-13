import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MagnetoService } from './services/magneto.service';
import {
  ProxyStatus,
  StartProxyRequest,
  CassetteInfo
} from './models/magneto.models';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="app">
      <header>
        <h1>🧲 Magneto-Serge Control Panel</h1>
        <p class="subtitle">Architecture Production: Angular + Backend Node.js</p>
      </header>

      <!-- Status Section -->
      <section class="status-section">
        <h2>📊 Proxy Status</h2>
        <div class="status-card" [class.running]="proxyStatus?.running">
          <div *ngIf="proxyStatus; else loadingStatus">
            <div class="status-grid">
              <div class="status-item">
                <span class="label">Status</span>
                <span class="value" [class.running]="proxyStatus.running">
                  {{ proxyStatus.running ? '🟢 Running' : '⚫ Stopped' }}
                </span>
              </div>
              <div class="status-item">
                <span class="label">Mode</span>
                <span class="value">{{ proxyStatus.mode }}</span>
              </div>
              <div class="status-item">
                <span class="label">Port</span>
                <span class="value">{{ proxyStatus.port }}</span>
              </div>
              <div class="status-item">
                <span class="label">Cassette</span>
                <span class="value">{{ proxyStatus.cassette || 'None' }}</span>
              </div>
              <div class="status-item">
                <span class="label">Interactions</span>
                <span class="value">{{ proxyStatus.interactions_count }}</span>
              </div>
              <div class="status-item">
                <span class="label">Uptime</span>
                <span class="value">{{ proxyStatus.uptime_seconds }}s</span>
              </div>
            </div>
          </div>
          <ng-template #loadingStatus>
            <p class="loading">Loading status...</p>
          </ng-template>
        </div>

        <button (click)="refreshStatus()" class="btn-refresh">
          🔄 Refresh Status
        </button>
      </section>

      <!-- Control Section -->
      <section class="control-section">
        <h2>⚙️ Proxy Control</h2>
        <div class="control-form">
          <div class="form-group">
            <label>Cassette Name</label>
            <input
              type="text"
              [(ngModel)]="startRequest.cassette_name"
              placeholder="my-test"
            />
          </div>

          <div class="form-group">
            <label>Mode</label>
            <select [(ngModel)]="startRequest.mode">
              <option value="auto">Auto (record if missing, else replay)</option>
              <option value="record">Record (always record)</option>
              <option value="replay">Replay (always replay)</option>
              <option value="passthrough">Passthrough (no recording)</option>
            </select>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Port</label>
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
          </div>

          <div class="button-group">
            <button
              (click)="startProxy()"
              [disabled]="loading || (proxyStatus && proxyStatus.running)"
              class="btn-start"
            >
              🟢 Start Proxy
            </button>
            <button
              (click)="stopProxy()"
              [disabled]="loading || !proxyStatus || !proxyStatus.running"
              class="btn-stop"
            >
              ⏹️ Stop Proxy
            </button>
          </div>
        </div>
      </section>

      <!-- Cassettes Section -->
      <section class="cassettes-section">
        <div class="section-header">
          <h2>📼 Cassettes ({{ cassettes.length }})</h2>
          <button (click)="loadCassettes()" class="btn-small">
            🔄 Refresh
          </button>
        </div>

        <div *ngIf="cassettes.length > 0; else noCassettes" class="cassettes-grid">
          <div *ngFor="let cassette of cassettes" class="cassette-card">
            <div class="cassette-header">
              <h3>{{ cassette.name }}</h3>
              <span class="cassette-format">{{ cassette.format }}</span>
            </div>
            <div class="cassette-details">
              <p><strong>Size:</strong> {{ formatSize(cassette.size_bytes) }}</p>
              <p><strong>Interactions:</strong> {{ cassette.interactions }}</p>
              <p><strong>Created:</strong> {{ formatDate(cassette.created_at) }}</p>
            </div>
            <div class="cassette-actions">
              <button
                (click)="deleteCassette(cassette.name)"
                class="btn-delete"
              >
                🗑️ Delete
              </button>
            </div>
          </div>
        </div>

        <ng-template #noCassettes>
          <p class="no-data">No cassettes found. Start recording to create one!</p>
        </ng-template>
      </section>

      <!-- Cache Section -->
      <section class="cache-section">
        <h2>💾 Backend Cache</h2>
        <div *ngIf="cacheStats" class="cache-info">
          <p><strong>Cached Resources:</strong> {{ cacheStats.cache?.size || 0 }}</p>
          <button (click)="clearBackendCache()" class="btn-clear">
            🗑️ Clear Cache
          </button>
        </div>
      </section>

      <!-- Error Display -->
      <div *ngIf="error" class="error-message">
        <strong>❌ Error:</strong> {{ error }}
        <button (click)="error = null" class="btn-close">✕</button>
      </div>

      <!-- Loading Overlay -->
      <div *ngIf="loading" class="loading-overlay">
        <div class="spinner"></div>
        <p>Loading...</p>
      </div>
    </div>
  `,
  styles: [`
    .app {
      max-width: 1200px;
      margin: 0 auto;
      padding: 20px;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    }

    header {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      padding: 40px;
      border-radius: 12px;
      margin-bottom: 30px;
      text-align: center;
    }

    header h1 {
      margin: 0 0 10px 0;
      font-size: 2.5em;
    }

    .subtitle {
      margin: 0;
      opacity: 0.9;
      font-size: 1.1em;
    }

    section {
      background: white;
      padding: 30px;
      margin-bottom: 20px;
      border-radius: 10px;
      box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    }

    h2 {
      margin-top: 0;
      color: #333;
      border-bottom: 3px solid #667eea;
      padding-bottom: 10px;
    }

    .section-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 20px;
    }

    .section-header h2 {
      margin: 0;
      border: none;
      padding: 0;
    }

    /* Status Card */
    .status-card {
      border: 3px solid #ddd;
      border-radius: 10px;
      padding: 25px;
      margin-bottom: 15px;
      transition: all 0.3s;
    }

    .status-card.running {
      border-color: #52c41a;
      background: #f6ffed;
    }

    .status-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 20px;
    }

    .status-item {
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    .status-item .label {
      font-size: 13px;
      color: #888;
      text-transform: uppercase;
      font-weight: 600;
      letter-spacing: 0.5px;
    }

    .status-item .value {
      font-size: 20px;
      font-weight: 700;
      color: #333;
    }

    .status-item .value.running {
      color: #52c41a;
    }

    /* Forms */
    .control-form {
      display: flex;
      flex-direction: column;
      gap: 20px;
    }

    .form-row {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 20px;
    }

    .form-group {
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    .form-group label {
      font-weight: 600;
      color: #555;
      font-size: 14px;
    }

    .form-group input,
    .form-group select {
      padding: 12px;
      border: 2px solid #e0e0e0;
      border-radius: 8px;
      font-size: 15px;
      transition: border-color 0.3s;
    }

    .form-group input:focus,
    .form-group select:focus {
      outline: none;
      border-color: #667eea;
    }

    .form-group input[type="checkbox"] {
      width: auto;
      margin-right: 8px;
    }

    /* Buttons */
    .button-group {
      display: flex;
      gap: 15px;
      margin-top: 10px;
    }

    button {
      padding: 14px 28px;
      border: none;
      border-radius: 8px;
      font-size: 15px;
      font-weight: 600;
      cursor: pointer;
      transition: all 0.3s;
    }

    button:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    .btn-start {
      background: #52c41a;
      color: white;
      flex: 1;
    }

    .btn-start:hover:not(:disabled) {
      background: #389e0d;
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(82, 196, 26, 0.3);
    }

    .btn-stop {
      background: #ff4d4f;
      color: white;
      flex: 1;
    }

    .btn-stop:hover:not(:disabled) {
      background: #cf1322;
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(255, 77, 79, 0.3);
    }

    .btn-refresh {
      background: #1890ff;
      color: white;
      width: 100%;
    }

    .btn-refresh:hover {
      background: #096dd9;
    }

    .btn-small {
      padding: 8px 16px;
      background: #f0f0f0;
      color: #333;
      font-size: 14px;
    }

    .btn-small:hover {
      background: #d9d9d9;
    }

    .btn-delete {
      background: #fff1f0;
      color: #ff4d4f;
      width: 100%;
      padding: 10px;
    }

    .btn-delete:hover {
      background: #ffccc7;
    }

    .btn-clear {
      background: #faad14;
      color: white;
      margin-top: 10px;
    }

    .btn-clear:hover {
      background: #d48806;
    }

    /* Cassettes Grid */
    .cassettes-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: 20px;
    }

    .cassette-card {
      border: 2px solid #f0f0f0;
      border-radius: 10px;
      padding: 20px;
      transition: all 0.3s;
    }

    .cassette-card:hover {
      border-color: #667eea;
      box-shadow: 0 4px 16px rgba(102, 126, 234, 0.2);
      transform: translateY(-2px);
    }

    .cassette-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15px;
    }

    .cassette-header h3 {
      margin: 0;
      color: #333;
      font-size: 18px;
    }

    .cassette-format {
      padding: 4px 10px;
      background: #f0f0f0;
      border-radius: 6px;
      font-size: 12px;
      font-weight: 600;
      text-transform: uppercase;
    }

    .cassette-details p {
      margin: 8px 0;
      font-size: 14px;
      color: #666;
    }

    .cassette-actions {
      margin-top: 15px;
    }

    /* Cache */
    .cache-info {
      padding: 20px;
      background: #f5f5f5;
      border-radius: 8px;
    }

    .cache-info p {
      margin: 0 0 10px 0;
      font-size: 16px;
    }

    /* Messages */
    .loading {
      text-align: center;
      color: #888;
      font-style: italic;
    }

    .no-data {
      text-align: center;
      color: #888;
      padding: 40px;
      font-size: 16px;
    }

    .error-message {
      position: fixed;
      top: 20px;
      right: 20px;
      background: #ff4d4f;
      color: white;
      padding: 20px;
      border-radius: 10px;
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
      max-width: 400px;
      z-index: 1000;
      animation: slideIn 0.3s;
    }

    @keyframes slideIn {
      from {
        transform: translateX(400px);
        opacity: 0;
      }
      to {
        transform: translateX(0);
        opacity: 1;
      }
    }

    .btn-close {
      background: none;
      color: white;
      padding: 0;
      margin-left: 15px;
      font-size: 20px;
      float: right;
    }

    .btn-close:hover {
      opacity: 0.8;
    }

    /* Loading Overlay */
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
      z-index: 999;
    }

    .spinner {
      border: 5px solid rgba(255, 255, 255, 0.3);
      border-top: 5px solid white;
      border-radius: 50%;
      width: 60px;
      height: 60px;
      animation: spin 1s linear infinite;
      margin-bottom: 20px;
    }

    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }

    /* Responsive */
    @media (max-width: 768px) {
      .app {
        padding: 10px;
      }

      header {
        padding: 20px;
      }

      header h1 {
        font-size: 1.8em;
      }

      section {
        padding: 20px;
      }

      .form-row {
        grid-template-columns: 1fr;
      }

      .button-group {
        flex-direction: column;
      }

      .cassettes-grid {
        grid-template-columns: 1fr;
      }
    }
  `]
})
export class AppComponent implements OnInit {
  // État du proxy
  proxyStatus: ProxyStatus | null = null;

  // Configuration de démarrage
  startRequest: StartProxyRequest = {
    mode: 'auto',
    cassette_name: 'my-test',
    port: 8888,
    strict: false
  };

  // Cassettes
  cassettes: CassetteInfo[] = [];

  // Cache
  cacheStats: any = null;

  // UI
  loading = false;
  error: string | null = null;

  constructor(private magnetoService: MagnetoService) {}

  ngOnInit(): void {
    this.refreshStatus();
    this.loadCassettes();
    this.loadCacheStats();
  }

  refreshStatus(): void {
    this.loading = true;
    this.magnetoService.getProxyStatus().subscribe({
      next: (response) => {
        this.proxyStatus = response.status!;
        this.loading = false;
      },
      error: (err) => {
        this.error = err.message;
        this.loading = false;
      }
    });
  }

  startProxy(): void {
    this.loading = true;
    this.magnetoService.startProxy(this.startRequest).subscribe({
      next: (response) => {
        this.loading = false;
        setTimeout(() => this.refreshStatus(), 1000);
      },
      error: (err) => {
        this.error = err.message;
        this.loading = false;
      }
    });
  }

  stopProxy(): void {
    this.loading = true;
    this.magnetoService.stopProxy().subscribe({
      next: (response) => {
        this.loading = false;
        setTimeout(() => this.refreshStatus(), 1000);
      },
      error: (err) => {
        this.error = err.message;
        this.loading = false;
      }
    });
  }

  loadCassettes(): void {
    this.magnetoService.listCassettes().subscribe({
      next: (response) => {
        this.cassettes = response.cassettes || [];
      },
      error: (err) => {
        this.error = err.message;
      }
    });
  }

  deleteCassette(name: string): void {
    if (!confirm(`Delete cassette "${name}"?`)) return;

    this.loading = true;
    this.magnetoService.deleteCassette(name).subscribe({
      next: () => {
        this.loading = false;
        this.loadCassettes();
      },
      error: (err) => {
        this.error = err.message;
        this.loading = false;
      }
    });
  }

  loadCacheStats(): void {
    this.magnetoService.getCacheStats().subscribe({
      next: (stats) => {
        this.cacheStats = stats;
      },
      error: () => {
        // Silent fail - cache stats non critique
      }
    });
  }

  clearBackendCache(): void {
    this.magnetoService.clearCache().subscribe({
      next: () => {
        this.loadCacheStats();
      },
      error: (err) => {
        this.error = err.message;
      }
    });
  }

  formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString();
  }
}
