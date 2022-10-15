import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SimControlsComponent } from './sim-controls.component';

describe('SimControlsComponent', () => {
  let component: SimControlsComponent;
  let fixture: ComponentFixture<SimControlsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SimControlsComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SimControlsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
