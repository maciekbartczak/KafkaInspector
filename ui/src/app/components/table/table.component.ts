import {Component, Input, OnInit} from '@angular/core';

interface TableData {
    [key: string]: string | number;
}

@Component({
    selector: 'ki-table',
    standalone: true,
    imports: [],
    templateUrl: './table.component.html',
    styleUrl: './table.component.css'
})
export class TableComponent implements OnInit {
    ngOnInit(): void {
        this.columns = Object.keys(this.tableData[0] ?? {});
    }

    @Input({required: true})
    tableData!: TableData[];

    columns: string[] = [];
}
