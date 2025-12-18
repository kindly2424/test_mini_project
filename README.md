Demo program : https://drive.google.com/file/d/1h1bWXPvWVnRZ_Ic-gjDi3gcHqEskywgO/view?usp=sharing

- **Hash Password (Backend)**  
  Password user disimpan dalam bentuk hash menggunakan **Argon2**.

- **Proteksi API (Backend)**  
  Endpoint CRUD **Barang** dilindungi menggunakan **JWT Token**.  
  Request tanpa token yang valid tidak dapat mengakses endpoint tersebut.

- **Proteksi Halaman (Frontend)**  
  Halaman **Master Barang** tidak dapat diakses langsung melalui URL.  
  User wajib **login terlebih dahulu** untuk mengakses halaman tersebut.

- **Upload File**  
  Sistem mendukung upload file dengan format:
  **pdf, jpg, jpeg, png, docx, txt**.  
  Ukuran maksimal file adalah **5 MB**.