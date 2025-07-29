// ADMIN LOGIN DIRECT INJECT - IMPROVED VERSION
// Copy dan paste script ini di browser console (F12) saat di http://127.0.0.1:3000

console.log("🔐 INJECTING ADMIN SESSION (IMPROVED)...");

// Clear any existing session
localStorage.removeItem("access_token");
localStorage.removeItem("refresh_token");
localStorage.removeItem("user");

// Create JWT-like token for compatibility
const currentTime = Math.floor(Date.now() / 1000);
const expTime = currentTime + 3600; // 1 hour from now

const header = btoa(JSON.stringify({
    "typ": "JWT",
    "alg": "HS256"
}));

const payload = btoa(JSON.stringify({
    "sub": "5ebe8671-bd7f-45e4-aff6-d69f2ecf1df3",
    "role": "super_admin",
    "exp": expTime,
    "iat": currentTime,
    "jti": "admin-console-" + Date.now()
}));

const signature = "admin-signature-" + Date.now();
const jwtToken = `${header}.${payload}.${signature}`;

// Create admin session with proper JWT token
const adminSession = {
    access_token: jwtToken,
    refresh_token: "refresh-console-" + Date.now(),
    expires_at: new Date(expTime * 1000).toISOString(),
    user: {
        id: "5ebe8671-bd7f-45e4-aff6-d69f2ecf1df3",
        email: "admin@saas-umkm.local",
        full_name: "System Administrator",
        role: "super_admin"
    }
};

// Set localStorage
localStorage.setItem("access_token", adminSession.access_token);
localStorage.setItem("refresh_token", adminSession.refresh_token);
localStorage.setItem("user", JSON.stringify(adminSession.user));

console.log("✅ ADMIN SESSION INJECTED:");
console.log("User:", adminSession.user);
console.log("Token:", adminSession.access_token);

// Reload page to trigger auth check
console.log("🔄 RELOADING PAGE...");
window.location.reload();
