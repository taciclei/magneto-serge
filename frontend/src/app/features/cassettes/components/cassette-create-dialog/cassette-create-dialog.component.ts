import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormBuilder, Validators } from '@angular/forms';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatSelectModule } from '@angular/material/select';
import { Store } from '@ngrx/store';
import { CassetteActions } from '../../state/cassette.actions';

/**
 * Dialog component for creating a new cassette
 *
 * Provides a form with validation for:
 * - Cassette name (required, alphanumeric + hyphens/underscores, 1-100 chars)
 * - Mode selection (auto, record, replay, passthrough)
 * - Optional description
 */
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
    MatSelectModule
  ],
  templateUrl: './cassette-create-dialog.component.html',
  styleUrl: './cassette-create-dialog.component.scss'
})
export class CassetteCreateDialogComponent {
  private fb = inject(FormBuilder);
  private store = inject(Store);
  private dialogRef = inject(MatDialogRef<CassetteCreateDialogComponent>);

  /**
   * Form for cassette creation
   * Validation rules match backend validation in src/api/validation.rs
   */
  createForm = this.fb.group({
    name: ['', [
      Validators.required,
      Validators.minLength(1),
      Validators.maxLength(100),
      Validators.pattern(/^[a-zA-Z0-9_-]+$/),
      this.noLeadingTrailingSpecialChars()
    ]],
    mode: ['auto', Validators.required],
    description: ['']
  });

  /**
   * Available proxy modes
   */
  modes = [
    { value: 'auto', label: 'Auto (record if missing, else replay)' },
    { value: 'record', label: 'Record' },
    { value: 'replay', label: 'Replay' },
    { value: 'passthrough', label: 'Passthrough' }
  ];

  /**
   * Custom validator: no leading/trailing hyphens or underscores
   */
  private noLeadingTrailingSpecialChars() {
    return (control: any) => {
      const value = control.value;
      if (!value) return null;

      const first = value.charAt(0);
      const last = value.charAt(value.length - 1);

      if (first === '-' || first === '_' || last === '-' || last === '_') {
        return { leadingTrailing: 'Name cannot start or end with hyphen or underscore' };
      }

      return null;
    };
  }

  /**
   * Get validation error message for name field
   */
  getNameErrorMessage(): string {
    const nameControl = this.createForm.get('name');

    if (nameControl?.hasError('required')) {
      return 'Cassette name is required';
    }
    if (nameControl?.hasError('minlength') || nameControl?.hasError('maxlength')) {
      return 'Name must be 1-100 characters';
    }
    if (nameControl?.hasError('pattern')) {
      return 'Use only alphanumeric characters, hyphens, and underscores';
    }
    if (nameControl?.hasError('leadingTrailing')) {
      return nameControl.errors?.['leadingTrailing'];
    }

    return '';
  }

  /**
   * Submit form and dispatch create action
   */
  onSubmit(): void {
    if (this.createForm.valid) {
      const formValue = this.createForm.value;

      this.store.dispatch(CassetteActions.createCassette({
        name: formValue.name!,
        mode: formValue.mode,
        description: formValue.description || undefined
      }));

      this.dialogRef.close();
    }
  }

  /**
   * Cancel and close dialog
   */
  onCancel(): void {
    this.dialogRef.close();
  }
}
