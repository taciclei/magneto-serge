import { Component, OnInit, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, Router } from '@angular/router';
import { Store } from '@ngrx/store';
import { Observable, Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatChipsModule } from '@angular/material/chips';
import { MatDividerModule } from '@angular/material/divider';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTabsModule } from '@angular/material/tabs';

import { CassetteActions } from '../../state/cassette.actions';
import {
  selectSelectedCassette,
  selectLoading,
  selectError
} from '../../state/cassette.selectors';
import { CassetteResource } from '../../../../core/models/cassette.model';

/**
 * Component affichant les détails d'une cassette
 *
 * Affiche les métadonnées et permet de naviguer vers les interactions
 */
@Component({
  selector: 'app-cassette-detail',
  standalone: true,
  imports: [
    CommonModule,
    MatCardModule,
    MatButtonModule,
    MatIconModule,
    MatChipsModule,
    MatDividerModule,
    MatProgressSpinnerModule,
    MatTabsModule
  ],
  templateUrl: './cassette-detail.component.html',
  styleUrl: './cassette-detail.component.scss'
})
export class CassetteDetailComponent implements OnInit, OnDestroy {
  cassette$: Observable<CassetteResource | null>;
  loading$: Observable<boolean>;
  error$: Observable<string | null>;

  private destroy$ = new Subject<void>();

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private store: Store
  ) {
    this.cassette$ = this.store.select(selectSelectedCassette);
    this.loading$ = this.store.select(selectLoading);
    this.error$ = this.store.select(selectError);
  }

  ngOnInit(): void {
    // Récupérer le nom depuis l'URL
    this.route.params
      .pipe(takeUntil(this.destroy$))
      .subscribe(params => {
        const name = params['name'];
        if (name) {
          this.store.dispatch(CassetteActions.loadCassetteDetail({ name }));
        }
      });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
    this.store.dispatch(CassetteActions.clearSelection());
  }

  /**
   * Retour à la liste des cassettes
   */
  goBack(): void {
    this.router.navigate(['/cassettes']);
  }

  /**
   * Navigue vers les interactions de la cassette
   */
  viewInteractions(cassette: CassetteResource): void {
    this.router.navigate(['/cassettes', cassette.name, 'interactions']);
  }

  /**
   * Recharge les détails de la cassette
   */
  reload(cassette: CassetteResource): void {
    this.store.dispatch(CassetteActions.loadCassetteDetail({ name: cassette.name }));
  }

  /**
   * Formate la taille en octets en KB/MB
   */
  formatSize(bytes: number): string {
    if (bytes < 1024) {
      return `${bytes} B`;
    } else if (bytes < 1024 * 1024) {
      return `${(bytes / 1024).toFixed(2)} KB`;
    } else {
      return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    }
  }

  /**
   * Formate la date ISO en format lisible
   */
  formatDate(isoDate: string): string {
    const date = new Date(isoDate);
    return date.toLocaleString('fr-FR', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    });
  }
}
