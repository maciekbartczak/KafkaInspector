import {ChangeDetectorRef, Component, OnInit} from '@angular/core';
import {AppStateService} from "../../app-state.service";
import {NgForOf} from "@angular/common";
import {Topic} from "../../types";

@Component({
  selector: 'app-topic-list',
  standalone: true,
  templateUrl: './topic-list.component.html',
  styleUrl: './topic-list.component.css',
})
export class TopicListComponent implements OnInit {

  public topics: Topic[] = [];

  constructor(public state: AppStateService, private changeDetector: ChangeDetectorRef) {
  }

  ngOnInit(): void {
      this.state.metadata$.subscribe(it => {
        this.topics = it.topics;
        this.changeDetector.detectChanges();
      });
    }
}