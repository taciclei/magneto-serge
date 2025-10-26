# Roadmap v0.5.0: Interface Rétro VHS Style Années 90 📼

**Version:** 0.5.0 - "VHS Rewind Edition"
**Thème:** Location de cassettes VHS + Magnétoscope rétro
**Inspiration:** Blockbuster, Club Vidéo, magnétoscopes Sony/Panasonic des années 90
**Status:** 📋 Design & Planning

---

## 🎨 Vision Design: "Retro VHS Rental Interface"

### Concept Principal
Une interface nostalgique qui transforme la gestion de cassettes HTTP en expérience de location de VHS:
- **Cassettes HTTP/WebSocket** = Cassettes VHS dans un vidéoclub
- **Player/Recorder** = Magnétoscope avec télécommande
- **Dashboard** = Étagères de vidéoclub
- **Templates** = "Programmes TV à enregistrer"
- **Proxy** = Antenne TV + Tuner

### Aesthetic Goals
- 🎬 Skeuomorphisme (simulation d'objets réels)
- 📺 Scanlines CRT / effet écran cathodique
- 🔊 Sons authentiques (clics, whirr du magnétoscope)
- 🎨 Palette années 90 (beige, gris, plastique noir)
- 💾 Textures (plastique, métal brossé, étiquettes VHS)
- ⚡ Animations rétro (transitions lentes, tracking VHS)

---

# Phase 1: Design System Rétro (1 semaine)

## Semaine 1: UI Kit Années 90

### 1.1 Palette de Couleurs "VCR Era"
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **1.1.1 Couleurs Principales**
  ```scss
  // Magnétoscope (plastique noir/gris)
  $vcr-black: #1a1a1a;           // Plastique noir brillant
  $vcr-dark-gray: #2d2d2d;       // Grilles de ventilation
  $vcr-gray: #4a4a4a;            // Boutons gris
  $vcr-light-gray: #8a8a8a;      // Métal brossé
  $vcr-beige: #d4c5a9;           // Plastique beige (Sony)

  // Écran LCD/LED (affichage digital vert)
  $lcd-green: #00ff00;           // Digits verts classiques
  $lcd-bg: #0a0a0a;              // Fond noir LCD
  $lcd-red: #ff0000;             // Indicateur REC
  $lcd-orange: #ff8c00;          // Pause/Play

  // Cassette VHS
  $vhs-label-white: #f5f5dc;     // Étiquette beige
  $vhs-plastic-black: #000000;   // Plastique cassette
  $vhs-tape-brown: #3d2817;      // Bande magnétique
  $vhs-sticker-blue: #4169e1;    // Autocollants
  $vhs-sticker-red: #dc143c;
  $vhs-sticker-yellow: #ffd700;

  // UI Accents
  $blockbuster-blue: #004b9b;    // Bleu Blockbuster
  $blockbuster-yellow: #ffd700;   // Jaune Blockbuster
  $rental-red: #c41e3a;          // Rouge "LOUÉE"
  $available-green: #2e8b57;     // Vert "DISPONIBLE"

  // Effets
  $scanline-alpha: 0.03;         // Lignes de balayage CRT
  $noise-alpha: 0.05;            // Grain vidéo
  $tracking-distortion: blur(0.5px); // Effet tracking VHS
  ```

- [ ] **1.1.2 Dégradés & Textures**
  ```scss
  // Gradient plastique brillant
  @mixin plastic-gloss {
    background: linear-gradient(
      180deg,
      lighten($vcr-black, 10%) 0%,
      $vcr-black 50%,
      darken($vcr-black, 5%) 100%
    );
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  // Texture métal brossé
  @mixin brushed-metal {
    background:
      linear-gradient(90deg, transparent 50%, rgba(255,255,255,0.03) 50%),
      $vcr-light-gray;
    background-size: 2px 100%;
  }

  // Scanlines CRT
  @mixin scanlines {
    &::after {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: repeating-linear-gradient(
        0deg,
        rgba(0, 0, 0, $scanline-alpha) 0px,
        transparent 1px,
        transparent 2px,
        rgba(0, 0, 0, $scanline-alpha) 3px
      );
      pointer-events: none;
    }
  }
  ```

### 1.2 Typographie Rétro
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **1.2.1 Fonts**
  ```scss
  // LCD Display (7-segment style)
  @import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700;900&display=swap');
  $font-lcd: 'Orbitron', monospace;

  // Interface labels (années 90)
  @import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');
  $font-pixelated: 'Press Start 2P', monospace;

  // Texte général (système classique)
  @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;600&display=swap');
  $font-mono: 'IBM Plex Mono', monospace;

  // Titres VHS (écriture manuscrite)
  @import url('https://fonts.googleapis.com/css2?family=Permanent+Marker&display=swap');
  $font-handwritten: 'Permanent Marker', cursive;
  ```

- [ ] **1.2.2 Text Effects**
  ```scss
  // LCD glow effect
  @mixin lcd-text {
    font-family: $font-lcd;
    color: $lcd-green;
    text-shadow:
      0 0 5px $lcd-green,
      0 0 10px $lcd-green,
      0 0 15px rgba($lcd-green, 0.5);
    letter-spacing: 0.2em;
  }

  // VHS tracking distortion
  @mixin vhs-glitch($offset: 2px) {
    position: relative;

    &::before {
      content: attr(data-text);
      position: absolute;
      left: -$offset;
      color: cyan;
      mix-blend-mode: screen;
    }

    &::after {
      content: attr(data-text);
      position: absolute;
      right: -$offset;
      color: red;
      mix-blend-mode: screen;
    }
  }
  ```

### 1.3 Composants de Base
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **1.3.1 VCR Button (bouton magnétoscope)**
  ```typescript
  @Component({
    selector: 'app-vcr-button',
    template: `
      <button class="vcr-button" [class.pressed]="isPressed">
        <div class="button-face">
          <span class="button-icon">{{ icon }}</span>
          <span class="button-label">{{ label }}</span>
        </div>
        <audio #clickSound src="assets/sounds/button-click.mp3"></audio>
      </button>
    `,
    styles: [`
      .vcr-button {
        @include plastic-gloss;
        width: 60px;
        height: 40px;
        border: 1px solid #000;
        border-radius: 3px;
        position: relative;
        cursor: pointer;
        transition: transform 0.1s;

        &:active, &.pressed {
          transform: translateY(2px);
          box-shadow: inset 0 2px 4px rgba(0,0,0,0.5);
        }

        .button-face {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          color: $vcr-light-gray;
          font-size: 10px;
        }

        .button-icon {
          font-size: 18px;
          margin-bottom: 2px;
        }
      }
    `]
  })
  export class VcrButtonComponent {
    @Input() icon: string = '▶';  // ▶, ⏸, ⏹, ⏪, ⏩, ⏺
    @Input() label: string = 'PLAY';
    @Output() pressed = new EventEmitter<void>();

    isPressed = false;

    @ViewChild('clickSound') clickSound!: ElementRef<HTMLAudioElement>;

    onClick() {
      this.isPressed = true;
      this.clickSound.nativeElement.play();
      setTimeout(() => this.isPressed = false, 100);
      this.pressed.emit();
    }
  }
  ```

- [ ] **1.3.2 LCD Display**
  ```typescript
  @Component({
    selector: 'app-lcd-display',
    template: `
      <div class="lcd-container">
        <div class="lcd-screen" [class.recording]="isRecording">
          <span class="lcd-text">{{ displayText }}</span>
          <span class="rec-indicator" *ngIf="isRecording">⏺ REC</span>
        </div>
      </div>
    `,
    styles: [`
      .lcd-container {
        background: $lcd-bg;
        padding: 8px 16px;
        border-radius: 4px;
        border: 2px solid $vcr-dark-gray;
        box-shadow: inset 0 0 10px rgba(0,0,0,0.8);
      }

      .lcd-screen {
        @include lcd-text;
        font-size: 24px;
        display: flex;
        align-items: center;
        gap: 20px;
        min-width: 200px;
        height: 40px;
      }

      .rec-indicator {
        color: $lcd-red;
        animation: blink 1s infinite;
      }

      @keyframes blink {
        0%, 50% { opacity: 1; }
        51%, 100% { opacity: 0; }
      }
    `]
  })
  export class LcdDisplayComponent {
    @Input() displayText: string = '00:00:00';
    @Input() isRecording: boolean = false;
  }
  ```

- [ ] **1.3.3 VHS Cassette Card**
  ```typescript
  @Component({
    selector: 'app-vhs-cassette',
    template: `
      <div class="vhs-cassette" [class.selected]="selected">
        <div class="cassette-body">
          <div class="cassette-window">
            <div class="tape-reel left"></div>
            <div class="tape-reel right"></div>
          </div>

          <div class="cassette-label">
            <div class="label-header">
              <span class="brand">MAGNETO-SERGE</span>
              <span class="duration">{{ duration }}</span>
            </div>

            <div class="label-title" [attr.data-text]="name">
              {{ name }}
            </div>

            <div class="label-metadata">
              <div class="metadata-item">
                <span class="icon">📅</span>
                <span>{{ recordedAt | date:'short' }}</span>
              </div>
              <div class="metadata-item">
                <span class="icon">🔢</span>
                <span>{{ interactionCount }} interactions</span>
              </div>
            </div>

            <div class="status-sticker" [class]="status">
              {{ status === 'rented' ? 'LOUÉE' : 'DISPONIBLE' }}
            </div>
          </div>
        </div>

        <div class="cassette-spine">
          <span class="spine-text">{{ name }}</span>
        </div>
      </div>
    `,
    styles: [`
      .vhs-cassette {
        width: 300px;
        height: 180px;
        position: relative;
        cursor: pointer;
        transition: transform 0.3s;

        &:hover {
          transform: translateY(-5px);
        }

        &.selected {
          box-shadow: 0 0 20px $blockbuster-yellow;
        }
      }

      .cassette-body {
        background: $vhs-plastic-black;
        border: 2px solid #111;
        border-radius: 4px;
        padding: 10px;
        box-shadow:
          inset 0 0 0 1px rgba(255,255,255,0.1),
          0 4px 8px rgba(0,0,0,0.5);
      }

      .cassette-window {
        width: 100%;
        height: 50px;
        background: rgba(0,0,0,0.8);
        border-radius: 2px;
        display: flex;
        justify-content: space-between;
        padding: 8px 20px;
        margin-bottom: 10px;
      }

      .tape-reel {
        width: 35px;
        height: 35px;
        border-radius: 50%;
        background: radial-gradient(
          circle,
          $vhs-tape-brown 30%,
          #1a1a1a 30%,
          #1a1a1a 70%,
          $vhs-tape-brown 70%
        );
        animation: rotate 4s linear infinite;

        &.right {
          animation-direction: reverse;
        }
      }

      @keyframes rotate {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
      }

      .cassette-label {
        background: $vhs-label-white;
        border: 1px solid #ccc;
        padding: 8px;
        font-family: $font-mono;
        font-size: 11px;
        position: relative;
      }

      .label-header {
        display: flex;
        justify-content: space-between;
        border-bottom: 1px dashed #999;
        padding-bottom: 4px;
        margin-bottom: 6px;
      }

      .brand {
        font-weight: bold;
        color: $blockbuster-blue;
        font-size: 9px;
      }

      .label-title {
        font-family: $font-handwritten;
        font-size: 16px;
        color: #000;
        margin-bottom: 8px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      .status-sticker {
        position: absolute;
        top: 8px;
        right: 8px;
        padding: 2px 6px;
        border-radius: 2px;
        font-size: 8px;
        font-weight: bold;
        transform: rotate(15deg);

        &.available {
          background: $available-green;
          color: white;
        }

        &.rented {
          background: $rental-red;
          color: white;
        }
      }

      .cassette-spine {
        position: absolute;
        left: -20px;
        top: 0;
        height: 100%;
        width: 20px;
        background: $vhs-plastic-black;
        border: 1px solid #111;
        writing-mode: vertical-lr;
        display: flex;
        align-items: center;
        justify-content: center;
      }

      .spine-text {
        font-size: 10px;
        color: white;
        font-family: $font-mono;
      }
    `]
  })
  export class VhsCassetteComponent {
    @Input() name: string = '';
    @Input() recordedAt: string = '';
    @Input() interactionCount: number = 0;
    @Input() duration: string = 'E-180';
    @Input() status: 'available' | 'rented' = 'available';
    @Input() selected: boolean = false;
    @Output() selected Change = new EventEmitter<void>();
  }
  ```

### 1.4 Effets Visuels Rétro
**Durée:** 1 jour
**Priorité:** 🟡 Important

- [ ] **1.4.1 CRT Scanlines Directive**
  ```typescript
  @Directive({
    selector: '[appCrtEffect]'
  })
  export class CrtEffectDirective implements AfterViewInit {
    constructor(private el: ElementRef) {}

    ngAfterViewInit() {
      this.el.nativeElement.style.position = 'relative';

      // Scanlines overlay
      const scanlines = document.createElement('div');
      scanlines.className = 'crt-scanlines';
      this.el.nativeElement.appendChild(scanlines);

      // Vignette effect
      const vignette = document.createElement('div');
      vignette.className = 'crt-vignette';
      this.el.nativeElement.appendChild(vignette);
    }
  }
  ```

  ```scss
  .crt-scanlines {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: repeating-linear-gradient(
      0deg,
      rgba(0, 0, 0, 0.03) 0px,
      transparent 1px,
      transparent 2px,
      rgba(0, 0, 0, 0.03) 3px
    );
    pointer-events: none;
    z-index: 9999;
  }

  .crt-vignette {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: radial-gradient(
      ellipse at center,
      transparent 60%,
      rgba(0, 0, 0, 0.4) 100%
    );
    pointer-events: none;
    z-index: 9998;
  }
  ```

- [ ] **1.4.2 VHS Tracking Glitch Animation**
  ```typescript
  @Component({
    selector: 'app-vhs-glitch',
    template: `
      <div class="glitch-container">
        <ng-content></ng-content>
        <div class="glitch-overlay" *ngIf="isGlitching"></div>
      </div>
    `,
    animations: [
      trigger('glitch', [
        transition(':enter', [
          style({ transform: 'translateX(0)' }),
          animate('50ms', style({ transform: 'translateX(-5px)' })),
          animate('50ms', style({ transform: 'translateX(5px)' })),
          animate('50ms', style({ transform: 'translateX(0)' })),
        ])
      ])
    ]
  })
  export class VhsGlitchComponent {
    isGlitching = false;

    triggerGlitch() {
      this.isGlitching = true;
      setTimeout(() => this.isGlitching = false, 150);
    }
  }
  ```

- [ ] **1.4.3 Sons Rétro**
  - [ ] `button-click.mp3` - Clic bouton magnétoscope
  - [ ] `eject.mp3` - Éjection cassette
  - [ ] `insert.mp3` - Insertion cassette
  - [ ] `play.mp3` - Démarrage lecture
  - [ ] `rewind.mp3` - Rembobinage rapide
  - [ ] `tracking-noise.mp3` - Bruit tracking VHS
  - [ ] `static.mp3` - Parasites TV

---

# Phase 2: Pages Principales (2 semaines)

## Semaine 2-3: Interfaces Thématiques

### 2.1 Dashboard "Vidéoclub"
**Durée:** 3 jours
**Priorité:** 🔴 Critique

- [ ] **2.1.1 Layout Étagères**
  ```typescript
  @Component({
    selector: 'app-videoclub-dashboard',
    template: `
      <div class="videoclub-interior" appCrtEffect>
        <!-- Enseigne néon -->
        <div class="neon-sign">
          <h1 class="neon-text" data-text="MAGNETO-SERGE">
            MAGNETO-SERGE
          </h1>
          <p class="neon-subtitle">📼 Location de Cassettes HTTP/WebSocket</p>
        </div>

        <!-- Étagères de cassettes -->
        <div class="shelves-container">
          <div class="shelf" *ngFor="let category of categories">
            <div class="shelf-label">
              <span class="category-icon">{{ category.icon }}</span>
              <span class="category-name">{{ category.name }}</span>
              <span class="category-count">({{ category.count }})</span>
            </div>

            <div class="shelf-content">
              <app-vhs-cassette
                *ngFor="let cassette of category.cassettes"
                [name]="cassette.name"
                [recordedAt]="cassette.recordedAt"
                [interactionCount]="cassette.interactionCount"
                [status]="cassette.status"
                (click)="selectCassette(cassette)">
              </app-vhs-cassette>
            </div>
          </div>
        </div>

        <!-- Comptoir (Stats) -->
        <div class="counter">
          <div class="counter-surface">
            <div class="stat-display">
              <app-lcd-display
                [displayText]="totalCassettes + ' CASSETTES'"
                class="counter-lcd">
              </app-lcd-display>

              <app-lcd-display
                [displayText]="totalInteractions + ' INTER.'"
                class="counter-lcd">
              </app-lcd-display>

              <app-lcd-display
                [displayText]="formatBytes(totalSize)"
                class="counter-lcd">
              </app-lcd-display>
            </div>
          </div>
        </div>
      </div>
    `,
    styles: [`
      .videoclub-interior {
        min-height: 100vh;
        background:
          linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        padding: 40px;
        position: relative;
      }

      .neon-sign {
        text-align: center;
        margin-bottom: 60px;
      }

      .neon-text {
        font-family: $font-pixelated;
        font-size: 48px;
        color: $blockbuster-yellow;
        text-shadow:
          0 0 10px $blockbuster-yellow,
          0 0 20px $blockbuster-yellow,
          0 0 30px $blockbuster-blue,
          0 0 40px $blockbuster-blue;
        animation: neon-flicker 3s infinite;
        @include vhs-glitch(3px);
      }

      @keyframes neon-flicker {
        0%, 100% { opacity: 1; }
        94% { opacity: 0.8; }
        95% { opacity: 1; }
        96% { opacity: 0.8; }
        97% { opacity: 1; }
      }

      .shelf {
        background: linear-gradient(
          180deg,
          #8b4513 0%,
          #654321 100%
        );
        border: 3px solid #3e2723;
        border-radius: 4px;
        margin-bottom: 40px;
        padding: 20px;
        box-shadow:
          0 4px 8px rgba(0,0,0,0.3),
          inset 0 2px 4px rgba(0,0,0,0.2);
      }

      .shelf-label {
        background: rgba(0,0,0,0.6);
        color: white;
        padding: 8px 16px;
        border-radius: 4px;
        margin-bottom: 20px;
        display: flex;
        align-items: center;
        gap: 12px;
        font-family: $font-mono;
      }

      .shelf-content {
        display: flex;
        gap: 20px;
        overflow-x: auto;
        padding: 10px;
        scroll-behavior: smooth;

        &::-webkit-scrollbar {
          height: 8px;
        }

        &::-webkit-scrollbar-track {
          background: rgba(0,0,0,0.3);
        }

        &::-webkit-scrollbar-thumb {
          background: $vcr-gray;
          border-radius: 4px;
        }
      }

      .counter {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        height: 100px;
        background: linear-gradient(
          180deg,
          #2d2d2d 0%,
          #1a1a1a 100%
        );
        border-top: 3px solid #000;
        box-shadow: 0 -4px 20px rgba(0,0,0,0.5);
      }

      .counter-surface {
        @include brushed-metal;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
      }

      .stat-display {
        display: flex;
        gap: 40px;
      }
    `]
  })
  export class VideoclubDashboardComponent {}
  ```

- [ ] **2.1.2 Catégories de cassettes**
  - [ ] 📺 "Nouveautés" (cassettes récentes)
  - [ ] ⭐ "Top Rated" (plus utilisées)
  - [ ] 🔴 "En cours d'enregistrement"
  - [ ] 📦 "Archive" (anciennes)
  - [ ] 🏷️ Par tags custom

### 2.2 Player Interface "Magnétoscope"
**Durée:** 3 jours
**Priorité:** 🔴 Critique

- [ ] **2.2.1 Télécommande Virtuelle**
  ```typescript
  @Component({
    selector: 'app-vcr-remote',
    template: `
      <div class="remote-control">
        <div class="remote-body">
          <!-- Marque -->
          <div class="remote-brand">
            <span>MAGNETO-SERGE</span>
            <span class="model">VCR-5000</span>
          </div>

          <!-- LCD Info -->
          <div class="remote-display">
            <app-lcd-display
              [displayText]="currentStatus"
              [isRecording]="mode === 'record'">
            </app-lcd-display>
          </div>

          <!-- Boutons principaux -->
          <div class="main-controls">
            <div class="control-row">
              <app-vcr-button
                icon="⏮"
                label="REW"
                (pressed)="rewind()">
              </app-vcr-button>

              <app-vcr-button
                icon="▶"
                label="PLAY"
                (pressed)="play()"
                [class.active]="mode === 'play'">
              </app-vcr-button>

              <app-vcr-button
                icon="⏭"
                label="FF"
                (pressed)="fastForward()">
              </app-vcr-button>
            </div>

            <div class="control-row">
              <app-vcr-button
                icon="⏹"
                label="STOP"
                (pressed)="stop()">
              </app-vcr-button>

              <app-vcr-button
                icon="⏸"
                label="PAUSE"
                (pressed)="pause()"
                [class.active]="mode === 'pause'">
              </app-vcr-button>

              <app-vcr-button
                icon="⏺"
                label="REC"
                (pressed)="record()"
                [class.active]="mode === 'record'"
                [class.recording]="mode === 'record'">
              </app-vcr-button>
            </div>
          </div>

          <!-- Boutons secondaires -->
          <div class="secondary-controls">
            <app-vcr-button
              icon="⏏"
              label="EJECT"
              (pressed)="eject()">
            </app-vcr-button>

            <app-vcr-button
              icon="🔄"
              label="REPLAY"
              (pressed)="setMode('replay')">
            </app-vcr-button>

            <app-vcr-button
              icon="↻"
              label="AUTO"
              (pressed)="setMode('auto')">
            </app-vcr-button>
          </div>

          <!-- Sélecteur de cassette -->
          <div class="cassette-selector">
            <label>CASSETTE:</label>
            <select [(ngModel)]="selectedCassette">
              <option *ngFor="let cassette of cassettes" [value]="cassette.name">
                {{ cassette.name }}
              </option>
            </select>
          </div>

          <!-- Tracking control (slider) -->
          <div class="tracking-control">
            <label>TRACKING</label>
            <input
              type="range"
              min="0"
              max="100"
              [(ngModel)]="tracking"
              (input)="adjustTracking($event)">
          </div>
        </div>
      </div>
    `,
    styles: [`
      .remote-control {
        width: 280px;
        position: fixed;
        right: 40px;
        top: 50%;
        transform: translateY(-50%);
        z-index: 1000;
      }

      .remote-body {
        @include plastic-gloss;
        background: $vcr-dark-gray;
        border-radius: 12px;
        padding: 20px;
        box-shadow:
          0 8px 16px rgba(0,0,0,0.4),
          inset 0 0 0 2px rgba(255,255,255,0.05);
      }

      .remote-brand {
        text-align: center;
        margin-bottom: 16px;
        font-family: $font-mono;

        span {
          display: block;
          color: $vcr-light-gray;
          font-size: 12px;
          font-weight: bold;
        }

        .model {
          font-size: 9px;
          opacity: 0.7;
        }
      }

      .remote-display {
        margin-bottom: 20px;
      }

      .main-controls {
        margin-bottom: 16px;
      }

      .control-row {
        display: flex;
        justify-content: space-between;
        gap: 8px;
        margin-bottom: 8px;

        app-vcr-button {
          flex: 1;

          &.active {
            border: 2px solid $lcd-orange;
          }

          &.recording {
            animation: rec-pulse 1s infinite;
          }
        }
      }

      @keyframes rec-pulse {
        0%, 100% {
          box-shadow: 0 0 5px $lcd-red;
        }
        50% {
          box-shadow: 0 0 20px $lcd-red;
        }
      }

      .secondary-controls {
        display: flex;
        gap: 8px;
        margin-bottom: 16px;

        app-vcr-button {
          flex: 1;
        }
      }

      .cassette-selector {
        margin-bottom: 12px;

        label {
          display: block;
          font-family: $font-mono;
          font-size: 9px;
          color: $vcr-light-gray;
          margin-bottom: 4px;
        }

        select {
          width: 100%;
          background: $vcr-black;
          color: $lcd-green;
          border: 1px solid #000;
          padding: 6px;
          font-family: $font-mono;
          font-size: 11px;
        }
      }

      .tracking-control {
        label {
          display: block;
          font-family: $font-mono;
          font-size: 9px;
          color: $vcr-light-gray;
          margin-bottom: 4px;
        }

        input[type="range"] {
          width: 100%;
          -webkit-appearance: none;
          background: transparent;

          &::-webkit-slider-track {
            background: $vcr-black;
            height: 4px;
            border: 1px solid #000;
          }

          &::-webkit-slider-thumb {
            -webkit-appearance: none;
            width: 12px;
            height: 20px;
            background: $vcr-light-gray;
            border: 1px solid #000;
            cursor: pointer;
          }
        }
      }
    `]
  })
  export class VcrRemoteComponent {
    mode: 'stop' | 'play' | 'pause' | 'record' | 'rewind' | 'ff' = 'stop';
    currentStatus = 'STOP';
    selectedCassette = '';
    tracking = 50;
    cassettes: any[] = [];

    @Output() modeChange = new EventEmitter<string>();

    play() {
      this.mode = 'play';
      this.currentStatus = 'PLAYING';
      this.playSound('play.mp3');
      this.modeChange.emit('play');
    }

    record() {
      this.mode = 'record';
      this.currentStatus = 'RECORDING';
      this.playSound('button-click.mp3');
      this.modeChange.emit('record');
    }

    stop() {
      this.mode = 'stop';
      this.currentStatus = 'STOP';
      this.playSound('button-click.mp3');
      this.modeChange.emit('stop');
    }

    // etc...
  }
  ```

- [ ] **2.2.2 Affichage Principal VCR**
  ```typescript
  @Component({
    selector: 'app-vcr-display',
    template: `
      <div class="vcr-screen" appCrtEffect>
        <div class="vcr-frame">
          <!-- Counter (Time code) -->
          <div class="timecode-display">
            <app-lcd-display
              [displayText]="timecode"
              class="timecode">
            </app-lcd-display>
          </div>

          <!-- Video area (interactions display) -->
          <div class="video-area">
            <div class="tracking-lines" *ngIf="showTrackingNoise"></div>

            <div class="interaction-viewer">
              <app-interaction-display
                [interaction]="currentInteraction"
                *ngIf="currentInteraction">
              </app-interaction-display>

              <div class="no-signal" *ngIf="!currentInteraction">
                <div class="static-noise"></div>
                <p>NO SIGNAL</p>
              </div>
            </div>
          </div>

          <!-- OSD (On-Screen Display) -->
          <div class="osd" *ngIf="showOsd">
            <span class="osd-icon">{{ osdIcon }}</span>
            <span class="osd-text">{{ osdText }}</span>
          </div>
        </div>
      </div>
    `,
    styles: [`
      .vcr-screen {
        width: 100%;
        height: calc(100vh - 100px);
        background: #000;
        position: relative;
        overflow: hidden;
      }

      .vcr-frame {
        width: 90%;
        height: 90%;
        margin: 5%;
        border: 20px solid $vcr-black;
        border-radius: 4px;
        background: #000;
        box-shadow:
          inset 0 0 20px rgba(0,0,0,0.8),
          0 0 40px rgba(0,0,0,0.5);
      }

      .timecode-display {
        position: absolute;
        top: 20px;
        right: 20px;
        z-index: 100;
      }

      .tracking-lines {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: repeating-linear-gradient(
          0deg,
          transparent 0px,
          transparent 2px,
          rgba(255,255,255,0.05) 2px,
          rgba(255,255,255,0.05) 4px
        );
        animation: tracking-roll 2s linear infinite;
        pointer-events: none;
      }

      @keyframes tracking-roll {
        from { transform: translateY(0); }
        to { transform: translateY(20px); }
      }

      .no-signal {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        position: relative;

        .static-noise {
          position: absolute;
          width: 100%;
          height: 100%;
          background-image: url('data:image/svg+xml;utf8,<svg...>'); // Noise pattern
          animation: static-flicker 0.1s infinite;
          opacity: 0.3;
        }

        p {
          @include lcd-text;
          font-size: 48px;
          z-index: 1;
        }
      }

      .osd {
        position: absolute;
        bottom: 40px;
        left: 40px;
        background: rgba(0,0,0,0.8);
        padding: 12px 20px;
        border-radius: 4px;
        display: flex;
        align-items: center;
        gap: 12px;
        font-family: $font-mono;
        color: white;
        font-size: 18px;
      }

      .osd-icon {
        font-size: 24px;
      }
    `]
  })
  export class VcrDisplayComponent {}
  ```

### 2.3 Template Editor "Programme TV"
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **2.3.1 Interface "Guide TV"**
  ```typescript
  @Component({
    selector: 'app-tv-guide-editor',
    template: `
      <div class="tv-guide-container">
        <div class="guide-header">
          <h2 class="guide-title">📺 GUIDE DES PROGRAMMES</h2>
          <p class="guide-subtitle">Configurez vos enregistrements automatiques</p>
        </div>

        <div class="guide-content">
          <!-- Liste des templates (programmes) -->
          <div class="program-list">
            <div class="program-card" *ngFor="let template of templates">
              <div class="program-time">
                <span class="channel">{{ template.category }}</span>
                <span class="duration">{{ template.duration }}</span>
              </div>

              <div class="program-info">
                <h3 class="program-title">{{ template.name }}</h3>
                <p class="program-description">{{ template.description }}</p>
              </div>

              <div class="program-actions">
                <button class="record-button" (click)="editTemplate(template)">
                  ⏺ PROGRAMMER
                </button>
              </div>
            </div>
          </div>

          <!-- Éditeur (télétexte style) -->
          <div class="teletext-editor" *ngIf="selectedTemplate">
            <div class="teletext-header">
              <span class="page-number">P100</span>
              <span class="page-title">ÉDITEUR DE TEMPLATE</span>
              <span class="date-time">{{ now | date:'dd/MM HH:mm' }}</span>
            </div>

            <div class="teletext-content">
              <textarea
                class="teletext-textarea"
                [(ngModel)]="selectedTemplate.syntax"
                [style.font-family]="'monospace'">
              </textarea>
            </div>

            <div class="teletext-footer">
              <span class="hint">F1: AIDE</span>
              <span class="hint">F2: HELPERS</span>
              <span class="hint">F3: PREVIEW</span>
              <span class="hint">F4: SAUVEGARDER</span>
            </div>
          </div>
        </div>
      </div>
    `,
    styles: [`
      .tv-guide-container {
        background: linear-gradient(
          180deg,
          #000051 0%,
          #000033 100%
        );
        min-height: 100vh;
        padding: 40px;
        font-family: $font-mono;
      }

      .guide-header {
        text-align: center;
        margin-bottom: 40px;
      }

      .guide-title {
        @include lcd-text;
        font-size: 36px;
        margin-bottom: 8px;
      }

      .program-card {
        background: linear-gradient(
          90deg,
          rgba(255,255,255,0.1) 0%,
          rgba(255,255,255,0.05) 100%
        );
        border: 2px solid $lcd-green;
        border-radius: 4px;
        padding: 16px;
        margin-bottom: 12px;
        display: grid;
        grid-template-columns: 120px 1fr auto;
        gap: 16px;
        align-items: center;
      }

      .program-time {
        display: flex;
        flex-direction: column;
        gap: 4px;

        .channel {
          @include lcd-text;
          font-size: 14px;
        }

        .duration {
          color: $lcd-orange;
          font-size: 12px;
        }
      }

      .teletext-editor {
        background: #000;
        border: 4px solid $lcd-green;
        padding: 20px;
        font-family: 'Courier New', monospace;
        color: $lcd-green;
      }

      .teletext-header {
        display: flex;
        justify-content: space-between;
        border-bottom: 2px solid $lcd-green;
        padding-bottom: 8px;
        margin-bottom: 16px;
        font-size: 14px;
      }

      .teletext-textarea {
        width: 100%;
        min-height: 400px;
        background: #000;
        color: $lcd-green;
        border: none;
        font-family: 'Courier New', monospace;
        font-size: 14px;
        resize: vertical;
        outline: none;

        &::selection {
          background: $lcd-green;
          color: #000;
        }
      }

      .teletext-footer {
        display: flex;
        gap: 20px;
        margin-top: 16px;
        font-size: 12px;
        color: $lcd-orange;

        .hint {
          padding: 4px 8px;
          background: rgba(255,140,0,0.2);
          border-radius: 2px;
        }
      }
    `]
  })
  export class TvGuideEditorComponent {}
  ```

---

# Phase 3: Interactions & Animations (1 semaine)

## Semaine 4: Polish & UX

### 3.1 Transitions VHS
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **3.1.1 Page Transitions**
  ```typescript
  // Transition "Éjection de cassette"
  export const ejectAnimation = trigger('eject', [
    transition('* => *', [
      query(':enter', [
        style({ transform: 'translateY(100%)' }),
        animate('600ms ease-out', style({ transform: 'translateY(0)' }))
      ], { optional: true }),
      query(':leave', [
        animate('600ms ease-in', style({ transform: 'translateY(-100%)' }))
      ], { optional: true })
    ])
  ]);

  // Transition "Tracking VHS"
  export const trackingAnimation = trigger('tracking', [
    transition(':enter', [
      style({
        filter: 'blur(4px)',
        transform: 'scaleY(0.9)'
      }),
      animate('400ms ease-out', style({
        filter: 'blur(0)',
        transform: 'scaleY(1)'
      }))
    ])
  ]);
  ```

### 3.2 Easter Eggs
**Durée:** 1 jour
**Priorité:** 🟢 Fun

- [ ] **3.2.1 "Be Kind Rewind"**
  - [ ] Message "Please rewind" après chaque replay
  - [ ] Animation de rembobinage automatique

- [ ] **3.2.2 Blockbuster Mode**
  - [ ] Thème Blockbuster (bleu/jaune)
  - [ ] "No late fees!" message

- [ ] **3.2.3 Snow Effect**
  - [ ] Neige TV sur "No Signal"
  - [ ] Son de parasites

### 3.3 Documentation Visuelle
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **3.3.1 Manuel utilisateur style années 90**
  - [ ] PDF avec design rétro
  - [ ] Screenshots interface
  - [ ] Guide de démarrage

- [ ] **3.3.2 Vidéo démo**
  - [ ] Screencast avec effet VHS
  - [ ] Musique synthwave
  - [ ] Sous-titres "DEMO TAPE"

---

# 📊 Assets à Créer

## Sons (.mp3)
- [ ] button-click.mp3
- [ ] eject.mp3
- [ ] insert.mp3
- [ ] play.mp3
- [ ] stop.mp3
- [ ] record.mp3
- [ ] rewind.mp3 (whirrrr)
- [ ] tracking-noise.mp3
- [ ] static.mp3
- [ ] tape-end.mp3 (clunk)

## Images
- [ ] vhs-texture.png
- [ ] plastic-pattern.png
- [ ] brushed-metal.png
- [ ] wood-shelf.jpg
- [ ] neon-glow.png
- [ ] static-noise.gif

## Fonts (Google Fonts)
- [ ] Orbitron (LCD)
- [ ] Press Start 2P (pixel)
- [ ] IBM Plex Mono
- [ ] Permanent Marker (handwritten)

---

# 🎯 Checklist Finale

## Design System
- [ ] Palette couleurs VHS complète
- [ ] Typographie rétro
- [ ] Composants de base (buttons, displays, cassettes)
- [ ] Effets visuels (CRT, scanlines, glitch)
- [ ] Sons authentiques

## Pages Principales
- [ ] Dashboard vidéoclub avec étagères
- [ ] Interface magnétoscope avec télécommande
- [ ] Template editor style télétexte
- [ ] Toutes les animations fluides

## Polish
- [ ] Transitions VHS
- [ ] Easter eggs
- [ ] Documentation visuelle
- [ ] Vidéo démo

---

**Prêt à créer l'interface la plus nostalgique de l'histoire des APIs! 📼✨**

Voulez-vous que je commence par créer les composants de base (VCR Button, LCD Display, VHS Cassette) ? 🎮
