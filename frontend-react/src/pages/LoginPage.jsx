import React, { useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { Alert, Snackbar } from "@mui/material";
import Login from "../components/Login";
import { login } from "../api/auth";

export default function LoginPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const successMessage = location.state?.successMessage;

  const [successOpen, setSuccessOpen] = useState(!!successMessage);
  const [errorOpen, setErrorOpen] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");


  const handleLogin = async (formData) => {
    try {
      const res = await login(formData);
      if (!res.success) {
        setErrorMessage(res.message);
        setErrorOpen(true);
      }
      else {
        const token = res?.data?.token;
        if (!token) {
          setErrorMessage("token tidak ditemukan");
          setErrorOpen(true);
          return;
        }
        localStorage.setItem("token", token);
        navigate("/masterbarang");
      }

    } catch (err) {
      setErrorMessage("Terjadi kesalahan pada server : " + err);
      setErrorOpen(true);
    }
  }

  return (
    <>
      <Login
        onSubmit={handleLogin}
      />

      <Snackbar
        open={successOpen}
        autoHideDuration={3000}
        onClose={() => setSuccessOpen(false)}
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      >
        <Alert severity="success" variant="filled">
          {successMessage}
        </Alert>
      </Snackbar>

      <Snackbar
        open={errorOpen}
        autoHideDuration={3000}
        onClose={() => setErrorOpen(false)}
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      >
        <Alert severity="error" variant="filled">
          {errorMessage}
        </Alert>
      </Snackbar>
    </>
  );
}
