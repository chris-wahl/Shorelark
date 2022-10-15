import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SimDisplayComponent } from './sim-display.component';

describe('SimDisplayComponent', () => {
  let component: SimDisplayComponent;
  let fixture: ComponentFixture<SimDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SimDisplayComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SimDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
