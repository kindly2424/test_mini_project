import React, { useEffect, useState } from "react";
import { Container, Snackbar, Alert } from "@mui/material";

import BarangForm from "../components/masterbarang/BarangForm";
import BarangTable from "../components/masterbarang/BarangTable";

import { listBarang, createBarang, updateBarang, deleteBarang, downloadBarangFile } from "../api/barang";

export default function MasterBarangPage() {
  const [items, setItems] = useState([]);
  const [mode, setMode] = useState("create");
  const [editingData, setEditingData] = useState(null);
  const [formKey, setFormKey] = useState(0);
  const [snack, setSnack] = useState({ open: false, msg: "", severity: "success" });

  useEffect(() => {
    fetchList();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function fetchList() {
    try {
      const res = await listBarang();
      if (res.success) setItems(res.data || []);
      else notify("error", res.message || "Gagal ambil data");
    } catch (err) {
      notify("error", String(err));
    }
  }

  function notify(severity, msg) {
    setSnack({ open: true, severity, msg });
  }

  const handleFormSubmit = async (formData) => {
    try {
      if (mode === "create") {
        const res = await createBarang(formData);
        if (res.success) {
          notify("success", res.message || "Berhasil tambah barang");
          setFormKey((k) => k + 1);
          await fetchList();
        } else notify("error", res.message || "Gagal tambah");
      } else if (mode === "edit" && editingData) {
        const res = await updateBarang(editingData.id_barang, formData);
        if (res.success) {
          notify("success", res.message || "Berhasil update barang");
          setMode("create");
          setEditingData(null);
          await fetchList();
        } else notify("error", res.message || "Gagal update");
      }
    } catch (err) {
      notify("error", String(err));
    }
  };

  const handleDelete = async (id) => {
    try {
      const res = await deleteBarang(id);
      if (res.success) {
        notify("success", res.message || "Berhasil hapus");
        await fetchList();
      } else notify("error", res.message || "Gagal hapus");
    } catch (err) {
      notify("error", String(err));
    }
  };

  const handleEditClick = (row) => {
    setMode("edit");
    setEditingData(row);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const handleCancel = () => {
    setMode("create");
    setEditingData(null);
    setFormKey((k) => k + 1);
  };

  const handleOpenFile = async (filename) => {
    if (!filename) {
      notify("error", "Tidak ada file");
      return;
    }
    try {
      const blob = await downloadBarangFile(filename);
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      setTimeout(() => window.URL.revokeObjectURL(url), 10000);
    } catch (err) {
      notify("error", "Gagal download file: " + String(err));
    }
  };

  return (
    <Container maxWidth="lg" sx={{ pt: 0 }}>
      <BarangForm
        key={formKey}
        mode={mode}
        initialData={editingData}
        onSubmit={handleFormSubmit}
        onCancel={handleCancel}
      />

      <BarangTable
        data={items}
        onEdit={handleEditClick}
        onDelete={handleDelete}
        onOpenFile={handleOpenFile}
      />

      <Snackbar
        open={snack.open}
        autoHideDuration={3000}
        onClose={() => setSnack({ ...snack, open: false })}
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      >
        <Alert severity={snack.severity} variant="filled">
          {snack.msg}
        </Alert>
      </Snackbar>
    </Container>
  );
}
