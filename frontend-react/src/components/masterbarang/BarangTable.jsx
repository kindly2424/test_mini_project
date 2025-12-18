import React from "react";
import { Box, IconButton, Tooltip, Typography } from "@mui/material";
import { DataGrid } from "@mui/x-data-grid";
import InsertDriveFileIcon from '@mui/icons-material/InsertDriveFile';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';

export default function BarangTable({ data = [], onEdit, onDelete, onOpenFile }) {

    const columns = [
        {
            field: "id_barang",
            headerName: "ID",
            width: 80,
        },
        {
            field: "nama_barang",
            headerName: "Nama",
            flex: 1,
        },
        {
            field: "harga_barang",
            headerName: "Harga (Rp)",
            headerAlign: "right",
            align: "right",
            width: 120,
            renderCell: (params) => {
                return params.value.toLocaleString("id-ID");
            },

        },
        {
            field: "stok_barang",
            headerName: "Stok",
            headerAlign: "right",
            align: "right",
            width: 100,
            renderCell: (params) => {
                return params.value.toLocaleString("id-ID");
            },
        },
        {
            field: "file_barang",
            headerName: "File",
            flex: 1,
            renderCell: (params) =>
                params.value ? params.value : "-",
        },
        {
            field: "aksi",
            headerName: "Aksi",
            headerAlign: "center",
            width: 150,
            sortable: false,
            filterable: false,
            renderCell: (params) => {
                const row = params.row;

                return (
                    <>
                        <Tooltip title="Download File">
                            <span>
                                <IconButton
                                    size="small"
                                    onClick={() => onOpenFile && onOpenFile(row.file_barang)}
                                    disabled={!row.file_barang}
                                >
                                    <InsertDriveFileIcon />
                                </IconButton>
                            </span>
                        </Tooltip>

                        <Tooltip title="Edit">
                            <span>
                                <IconButton
                                    size="small"
                                    onClick={() => onEdit && onEdit(row)}
                                >
                                    <EditIcon />
                                </IconButton>
                            </span>
                        </Tooltip>

                        <Tooltip title="Hapus">
                            <span>
                                <IconButton
                                    size="small"
                                    color="error"
                                    onClick={() => {
                                        if (window.confirm("Yakin ingin menghapus barang ini?")) {
                                            onDelete && onDelete(row.id_barang);
                                        }
                                    }}
                                >
                                    <DeleteIcon />
                                </IconButton>
                            </span>
                        </Tooltip>

                    </>
                );
            },
        },
    ];

    return (
        <Box sx={{ mt: 3 }}>
            <Typography variant="h6" sx={{ mb: 1 }}>
                Daftar Barang
            </Typography>

            <DataGrid
                rows={data}
                columns={columns}
                getRowId={(row) => row.id_barang}
                autoHeight
                pageSizeOptions={[5, 10, 20]}
                initialState={{
                    pagination: {
                        paginationModel: { pageSize: 5, page: 0 },
                    },
                }}
                disableRowSelectionOnClick
            />
        </Box>
    );
}
