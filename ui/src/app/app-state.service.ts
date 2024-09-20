import { Injectable } from "@angular/core";
import { BehaviorSubject, Observable, from } from "rxjs";

@Injectable({
  providedIn: "root",
})
export class AppStateService {
  private isConnectedSubject = new BehaviorSubject<boolean>(false);
  public isConnected$: Observable<boolean> =
    this.isConnectedSubject.asObservable();

  setIsConnected(isConnected: boolean) {
    this.isConnectedSubject.next(isConnected);
  }

  getIsConnected() {
    return this.isConnectedSubject.value;
  }
}
