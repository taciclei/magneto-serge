import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { Store } from '@ngrx/store';
import { of } from 'rxjs';
import { map, catchError, switchMap, withLatestFrom } from 'rxjs/operators';

import { AlcaeusService } from '../../../core/services/alcaeus.service';
import { CassetteActions } from './cassette.actions';
import { selectPage, selectLimit } from './cassette.selectors';
import { CassetteCollection, CassetteResource } from '../../../core/models/cassette.model';

/**
 * Effets NgRx pour les cassettes
 *
 * Gère les side-effects comme les appels HTTP via Alcaeus
 */
@Injectable()
export class CassetteEffects {
  constructor(
    private actions$: Actions,
    private alcaeusService: AlcaeusService,
    private store: Store
  ) {}

  /**
   * Effet: Charger les cassettes
   *
   * Déclenché par: loadCassettes, navigateToPage, navigateToNextPage, etc.
   */
  loadCassettes$ = createEffect(() =>
    this.actions$.pipe(
      ofType(
        CassetteActions.loadCassettes,
        CassetteActions.navigateToPage,
        CassetteActions.navigateToNextPage,
        CassetteActions.navigateToPreviousPage,
        CassetteActions.navigateToFirstPage,
        CassetteActions.navigateToLastPage
      ),
      withLatestFrom(
        this.store.select(selectPage),
        this.store.select(selectLimit)
      ),
      switchMap(([action, page, limit]) => {
        // Construire l'URL avec pagination
        const url = `/cassettes?page=${page}&limit=${limit}`;

        return this.alcaeusService.loadResource<CassetteCollection>(url).pipe(
          map(response => {
            const collection = response.root as CassetteCollection;
            const cassettes = collection['hydra:member'] || [];
            const totalItems = collection['hydra:totalItems'] || 0;

            return CassetteActions.loadCassettesSuccess({
              cassettes,
              totalItems,
              page,
              limit
            });
          }),
          catchError(error =>
            of(CassetteActions.loadCassettesFailure({
              error: error.message || 'Failed to load cassettes'
            }))
          )
        );
      })
    )
  );

  /**
   * Effet: Charger une cassette individuelle
   */
  loadCassetteDetail$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.loadCassetteDetail),
      switchMap(({ name }) => {
        const url = `/cassettes/${name}`;

        return this.alcaeusService.loadResource<CassetteResource>(url).pipe(
          map(response => {
            const cassette = response.root as CassetteResource;
            return CassetteActions.loadCassetteDetailSuccess({ cassette });
          }),
          catchError(error =>
            of(CassetteActions.loadCassetteDetailFailure({
              error: error.message || `Failed to load cassette: ${name}`
            }))
          )
        );
      })
    )
  );

  /**
   * Effet: Créer une nouvelle cassette
   */
  createCassette$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.createCassette),
      switchMap(({ name, mode, description }) => {
        const url = `/cassettes`;
        const data = { name, mode: mode || 'auto', description };

        return this.alcaeusService.createResource<CassetteResource>(url, data).pipe(
          map(response => {
            const cassette = response.root as CassetteResource;
            return CassetteActions.createCassetteSuccess({ cassette });
          }),
          catchError(error =>
            of(CassetteActions.createCassetteFailure({
              error: error.message || `Failed to create cassette: ${name}`
            }))
          )
        );
      })
    )
  );

  /**
   * Effet: Rafraîchir la liste après création réussie
   */
  refreshAfterCreate$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.createCassetteSuccess),
      map(() => CassetteActions.loadCassettes({}))
    )
  );

  /**
   * Effet: Mettre à jour une cassette
   */
  updateCassette$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.updateCassette),
      switchMap(({ name, description }) => {
        const url = `/cassettes/${name}`;
        const data = { description };

        return this.alcaeusService.updateResource<CassetteResource>(url, data).pipe(
          map(response => {
            const cassette = response.root as CassetteResource;
            return CassetteActions.updateCassetteSuccess({ cassette });
          }),
          catchError(error =>
            of(CassetteActions.updateCassetteFailure({
              error: error.message || `Failed to update cassette: ${name}`
            }))
          )
        );
      })
    )
  );

  /**
   * Effet: Rafraîchir la liste après mise à jour réussie
   */
  refreshAfterUpdate$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.updateCassetteSuccess),
      map(() => CassetteActions.loadCassettes({}))
    )
  );

  /**
   * Effet: Supprimer une cassette
   */
  deleteCassette$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.deleteCassette),
      switchMap(({ name }) => {
        const url = `/cassettes/${name}`;

        return this.alcaeusService.deleteResource(url).pipe(
          map(() => CassetteActions.deleteCassetteSuccess({ name })),
          catchError(error =>
            of(CassetteActions.deleteCassetteFailure({
              error: error.message || `Failed to delete cassette: ${name}`
            }))
          )
        );
      })
    )
  );

  /**
   * Effet: Rafraîchir la liste après suppression réussie
   */
  refreshAfterDelete$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.deleteCassetteSuccess),
      map(() => CassetteActions.loadCassettes({}))
    )
  );
}
