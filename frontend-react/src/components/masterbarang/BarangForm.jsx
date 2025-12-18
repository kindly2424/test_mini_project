import React, { useEffect, useState } from "react";
import { Box, TextField, Button, Typography } from "@mui/material";

export default function BarangForm({ mode = "create", initialData = null, onSubmit, onCancel }) {
    const [id, setId] = useState("");
    const [nama, setNama] = useState("");
    const [harga, setHarga] = useState("");
    const [stok, setStok] = useState("");
    const [file, setFile] = useState(null);

    const formatRupiah = (digits) => {
        if (!digits) return "";
        const n = Number(digits);
        if (Number.isNaN(n)) return "";
        return new Intl.NumberFormat("id-ID").format(n);
    };

    useEffect(() => {
        if (initialData) {
            // eslint-disable-next-line react-hooks/set-state-in-effect
            setId(initialData.id_barang ?? "");
            setNama(initialData.nama_barang ?? "");
            setHarga(initialData.harga_barang ?? "");
            setStok(initialData.stok_barang ?? "");
            // file name only for info; actual upload handled via file input
            setFile(null);
        } else {
            // reset
            setId("");
            setNama("");
            setHarga("");
            setStok("");
            setFile(null);
        }
    }, [initialData]);

    const handleFileChange = (e) => {
        const f = e.target.files?.[0] ?? null;
        setFile(f);
    };

    const handleSubmit = (e) => {
        e.preventDefault();

        const hargaNumber = parseInt(harga, 10);
        const stokNumber = parseInt(stok, 10);

        if (!nama.trim()) {
            alert("Nama barang wajib diisi");
            return;
        }
        if (isNaN(hargaNumber) || hargaNumber < 0) {
            alert("Harga harus diisi dan tidak boleh minus0");
            return;
        }

        if (isNaN(stokNumber) || stokNumber < 0) {
            alert("Stok harus diisi dan tidak boleh minus");
            return;
        }

        const fd = new FormData();
        fd.append("nama_barang", nama);
        fd.append("harga_barang", hargaNumber);
        fd.append("stok_barang", stokNumber);
        if (file) {
            fd.append("file_barang", file);
        }

        if (onSubmit) {
            onSubmit(fd);
        }
    };

    return (
        <Box component="form" onSubmit={handleSubmit} sx={{ mb: 3 }}>
            <Typography variant="h6" sx={{ mb: 1 }}>
                {mode === "create" ? "Tambah Barang" : "Edit Barang"}
            </Typography>

            <Box sx={{ display: "flex", gap: 1, mb: 1 }}>
                <TextField
                    label="ID"
                    value={id}
                    size="small"
                    margin="dense"
                    sx={{ width: 120 }}
                    disabled
                />

                <TextField
                    label="Nama Barang"
                    value={nama}
                    onChange={(e) => setNama(e.target.value)}
                    size="small"
                    margin="dense"
                    fullWidth
                    required
                />
            </Box>

            <Box sx={{ display: "flex", gap: 1, mb: 1 }}>
                <TextField
                    label="Harga (Rp)"
                    value={formatRupiah(harga)}
                    onChange={(e) => {
                        const rawValue = e.target.value.replace(/\./g, "");
                        setHarga(rawValue);
                    }}
                    size="small"
                    margin="dense"
                    sx={{ flex: 1 }}
                    required
                    inputProps={{
                        inputMode: "numeric",
                    }}
                />

                <TextField
                    label="Stok"
                    value={formatRupiah(stok)}
                    onChange={(e) => {
                        const rawValue = e.target.value.replace(/\./g, "");
                        setStok(rawValue);
                    }}
                    size="small"
                    margin="dense"
                    sx={{ flex: 1 }}
                    required
                    inputProps={{
                        inputMode: "numeric",
                    }}
                />
            </Box>

            <Box sx={{ mt: 1, mb: 1 }}>
                <input type="file" onChange={handleFileChange} />
                {initialData?.file_barang && (
                    <Typography variant="caption" display="block">
                        File saat ini: {initialData.file_barang}
                    </Typography>
                )}
            </Box>

            <Box sx={{ display: "flex", gap: 1 }}>
                <Button type="submit" variant="contained">
                    {mode === "create" ? "Create" : "Edit"}
                </Button>

                <Button type="button" variant="outlined" onClick={() => onCancel && onCancel()}>
                    Batal
                </Button>
            </Box>
        </Box>
    );
}
