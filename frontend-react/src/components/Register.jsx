import React, { useState } from "react";
import { TextField, Button, Box, Typography, Link } from "@mui/material";
import { Link as RouterLink } from "react-router-dom";

export default function Register({ onSubmit }) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (onSubmit) {
      onSubmit({username,password});
    }
  };

  return (
    <Box
      component="form"
      onSubmit={handleSubmit}
      sx={{ maxWidth: 400, mx: "auto", mt: 10, display: "flex", flexDirection: "column", gap: 2 }}
    >
      <Typography variant="h5" textAlign="center">Register</Typography>
      <TextField label="Username" type="text" value={username} onChange={(e) => setUsername(e.target.value)} required />
      <TextField label="Password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
      <Button variant="contained" type="submit">Register</Button>
      <Typography variant="body2" textAlign="center">
        Sudah punya akun?{" "}
        <Link
          component={RouterLink}
          to="/login"
          underline="hover"
          sx={{ cursor: "pointer" }}
        >
          Login
        </Link>
      </Typography>
    </Box>
  );
}
