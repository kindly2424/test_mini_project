import React, { useState } from "react";
import Register from "../components/Register";
import { useNavigate } from "react-router-dom";
import { Alert, Snackbar } from "@mui/material";
import { register } from "../api/auth";

export default function RegisterPage() {
  const navigate = useNavigate();
  const [errorOpen, setErrorOpen] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");
  const handleRegister = async (formData) => {
    try {
      const res = await register(formData);
      if (!res.success) {
        setErrorMessage(res.message);
        setErrorOpen(true);
      }
      else {
        navigate("/login", {
          state: {
            successMessage: "Register berhasil! Silakan login."
          }
        });
      }

    } catch (err) {
      setErrorMessage("Terjadi kesalahan pada server : " + err);
      setErrorOpen(true);
    }
  }

  return (
    <>
      <Register
        onSubmit={handleRegister}
      />

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
