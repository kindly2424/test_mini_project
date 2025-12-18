import React, { useState } from "react";
import { TextField, Button, Box, Typography, Link } from "@mui/material";
import { Link as RouterLink } from "react-router-dom";

export default function Login({ onSubmit }) {
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
      <Typography variant="h5" textAlign="center">Login</Typography>
      <TextField label="Username" type="text" value={username} onChange={(e) => setUsername(e.target.value)} required />
      <TextField label="Password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
      <Button variant="contained" type="submit">Login</Button>
      <Typography variant="body2" textAlign="center">
        Belum punya akun?{" "}
        <Link
          component={RouterLink}
          to="/register"
          underline="hover"
        >
          Register
        </Link>
      </Typography>
    </Box>
  );
}
