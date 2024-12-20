type Table = {
    name: string;
    fields: Array<Field>;
    data: Array<RowData>;
    active?: boolean;
};

type Field = {
    name: string;
    field_type: number | string | boolean | Blob;
};

type RowData = {
    row_id: string;
    data: Array<FieldData>;
};

type FieldData = {
    name: string;
    data: number | string | boolean | Blob | null;
};