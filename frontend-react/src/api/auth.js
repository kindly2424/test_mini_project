const BASE_URL = "http://127.0.0.1:3000";

export async function login(formData) {
  const res = await fetch(`${BASE_URL}/login/user`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
     body: JSON.stringify(formData),
  });
  return res.json();
}

export const register = async (formData) => {
  const res = await fetch("http://127.0.0.1:3000/register/user", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(formData),
  });
  return res.json();
};
