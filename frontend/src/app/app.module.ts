import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { SimDisplayComponent } from './components/sim-display/sim-display.component';
import { SimControlsComponent } from './components/sim-controls/sim-controls.component';
import {MaterialModule} from './modules/material/material.module';


@NgModule({
  declarations: [
    AppComponent,
    SimDisplayComponent,
    SimControlsComponent
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    MaterialModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
