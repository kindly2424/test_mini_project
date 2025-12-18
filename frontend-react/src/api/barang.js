const BASE_URL = "http://127.0.0.1:3000";

function authHeader() {
  const token = localStorage.getItem("token");
  return token ? { Authorization: `Bearer ${token}` } : {};
}

export async function listBarang() {
  const res = await fetch(`${BASE_URL}/barang`, {
    headers: {
      ...authHeader(),
    },
  });
  return res.json();
}

export async function createBarang(formData) {
  const res = await fetch(`${BASE_URL}/barang`, {
    method: "POST",
    headers: {
      ...authHeader(), 
    },
    body: formData,
  });
  return res.json();
}

export async function updateBarang(id, formData) {
  const res = await fetch(`${BASE_URL}/barang/${id}`, {
    method: "PUT",
    headers: {
      ...authHeader(),
    },
    body: formData,
  });
  return res.json();
}

export async function deleteBarang(id) {
  const res = await fetch(`${BASE_URL}/barang/${id}`, {
    method: "DELETE",
    headers: {
      ...authHeader(),
    },
  });
  return res.json();
}

export async function downloadBarangFile(filename) {
  const res = await fetch(`${BASE_URL}/barang/file/${encodeURIComponent(filename)}`, {
    headers: {
      ...authHeader(),
    },
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Gagal download file: ${res.status} ${text}`);
  }

  const blob = await res.blob();
  return blob;
}
