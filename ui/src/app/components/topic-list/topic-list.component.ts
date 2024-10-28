import {ChangeDetectorRef, Component, OnInit} from '@angular/core';
import {AppStateService} from "../../app-state.service";
import {Topic} from "../../types";
import {TableComponent} from "../table/table.component";

@Component({
    selector: 'app-topic-list',
    standalone: true,
    templateUrl: './topic-list.component.html',
    styleUrl: './topic-list.component.css',
    imports: [
        TableComponent
    ]
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