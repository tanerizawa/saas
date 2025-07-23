# Standar Kode dan Konvensi

Dokumen ini menjelaskan standar kode dan konvensi yang harus diikuti untuk pengembangan platform SaaS UMKM. Konsistensi dalam penulisan kode sangat penting untuk memastikan kualitas, pemeliharaan, dan kolaborasi yang efektif.

## Umum

### Penamaan

- Gunakan **snake_case** untuk nama file dan fungsi di Rust
- Gunakan **camelCase** untuk nama variabel dan fungsi di JavaScript/TypeScript
- Gunakan **PascalCase** untuk nama komponen React dan tipe/interface TypeScript
- Gunakan **SCREAMING_SNAKE_CASE** untuk konstanta

### Formatting

- Gunakan 4 spasi untuk indentasi di Rust
- Gunakan 2 spasi untuk indentasi di JavaScript/TypeScript
- Gunakan EOL (End of Line) karakter LF (Line Feed)
- Hindari trailing whitespace dan jaga konsistensi line endings

### Dokumentasi

- Dokumentasikan semua fungsi publik dan tipe data
- Berikan contoh penggunaan untuk API yang kompleks
- Gunakan komentar yang bermakna untuk menjelaskan "mengapa", bukan "apa"

## Backend (Rust)

### Struktur Kode

Arsitektur backend mengikuti prinsip Domain-Driven Design dalam konteks monolitik:

```
backend/
├── src/
│   ├── domain/          # Model domain bisnis (entities, value objects)
│   ├── application/     # Layanan aplikasi dan use case
│   ├── infrastructure/  # Implementasi teknis (database, messaging, dll)
│   └── presentation/    # API endpoints dan controllers
```

### Konvensi Rust

1. **Hasil Error**

   - Gunakan Result<T, E> untuk handling error
   - Gunakan tipe error custom dengan trait From untuk konversi
   - Hindari panic! kecuali untuk kondisi yang benar-benar tidak dapat dipulihkan

2. **Traits**

   - Implementasikan traits yang umum seperti Display, Debug, dll
   - Gunakan traits untuk abstraksi dependencies

3. **Testing**

   - Unit test untuk semua fungsi publik
   - Integration test untuk flows utama
   - Mock dependencies menggunakan traits

4. **Dokumentasi**
   - Semua fungsi publik harus memiliki doc-comments (///)
   - Berikan contoh penggunaan jika kompleks

### Example Rust Code

```rust
/// Represents a user in the system
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: UserId,
    name: String,
    email: Email,
    // Other fields...
}

impl User {
    /// Creates a new user
    ///
    /// # Arguments
    /// * `name` - The user's full name
    /// * `email` - The user's email address
    ///
    /// # Returns
    /// A Result containing the new User if valid, or a ValidationError
    pub fn new(name: String, email: String) -> Result<Self, ValidationError> {
        // Validation logic...
        let email = Email::try_from(email)?;

        Ok(Self {
            id: UserId::new(),
            name,
            email,
            // Other fields...
        })
    }

    // Other methods...
}
```

## Frontend (Next.js/React)

### Struktur Kode

```
frontend/
├── src/
│   ├── app/           # Next.js App Router pages
│   ├── components/    # Reusable UI components
│   │   ├── ui/        # Base UI components
│   │   └── features/  # Feature-specific components
│   ├── hooks/         # Custom React hooks
│   ├── lib/           # Utilities and API client
│   └── types/         # TypeScript type definitions
```

### Konvensi React/TypeScript

1. **Komponen**

   - Gunakan Function Components dengan hooks
   - Props harus memiliki interface TypeScript
   - Pisahkan komponen kompleks menjadi komponen yang lebih kecil

2. **State Management**

   - Gunakan React Context untuk state global
   - Gunakan useState/useReducer untuk state lokal
   - Hindari state yang tidak perlu

3. **Styling**

   - Gunakan TailwindCSS untuk styling komponen
   - Konsisten dalam penggunaan warna dan spacing

4. **Testing**
   - Unit test untuk komponen UI
   - Test interaksi pengguna dengan React Testing Library

### Example React Code

```tsx
interface UserProfileProps {
  userId: string;
  showDetails?: boolean;
}

export function UserProfile({ userId, showDetails = false }: UserProfileProps) {
  const { data: user, isLoading, error } = useUser(userId);

  if (isLoading) return <Spinner />;
  if (error) return <ErrorMessage message={error.message} />;

  return (
    <div className="p-4 bg-white rounded shadow">
      <h2 className="text-xl font-bold">{user.name}</h2>
      {showDetails && (
        <div className="mt-2">
          <p>Email: {user.email}</p>
          {/* More user details */}
        </div>
      )}
    </div>
  );
}
```

## API Communication

### REST API

- Gunakan kebab-case untuk URL paths
- Gunakan plural nouns untuk resource collections
- Gunakan status HTTP yang tepat
- Berikan error responses yang jelas dengan detail kesalahan

### Response Format

```json
{
  "success": true,
  "data": {
    // Response data here
  },
  "meta": {
    "pagination": {
      "page": 1,
      "perPage": 10,
      "total": 100
    }
  }
}
```

### Error Format

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input provided",
    "details": [
      {
        "field": "email",
        "message": "Must be a valid email address"
      }
    ]
  }
}
```

## Versioning dan Git

### Commit Messages

Format Commit:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:

- feat: Fitur baru
- fix: Bug fix
- docs: Perubahan dokumentasi
- style: Perubahan yang tidak memengaruhi kode (format, spasi)
- refactor: Refactoring kode
- perf: Peningkatan performa
- test: Menambah atau memperbaiki test
- chore: Perubahan pada build tools, dependencies, dll

### Branching

- `main`: Branch utama, selalu dalam kondisi stabil
- `feature/*`: Branch untuk pengembangan fitur
- `bugfix/*`: Branch untuk perbaikan bug
- `hotfix/*`: Branch untuk perbaikan darurat ke production

## Kesimpulan

Standar kode ini dirancang untuk mendukung arsitektur monolitik yang sudah ditetapkan. Dengan mengikuti konvensi ini, kode akan lebih mudah dipahami, dipelihara, dan dikelola oleh semua anggota tim.

---

_Dokumen Terakhir Diperbarui: 28 Juli 2025_
