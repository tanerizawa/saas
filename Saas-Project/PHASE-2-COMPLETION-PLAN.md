# PHASE-2-COMPLETION-PLAN

## 🎯 License Management Implementation Plan

Berikut adalah rencana konkrit untuk menyelesaikan Phase 2 - License Management Core yang sudah mencapai progress 75% dengan backend yang sudah siap.

### 📊 Current Status

- ✅ Domain entities & database schema completed
- ✅ License Repository implementation done
- ✅ API handlers dengan endpoints lengkap
- ✅ Backend business logic for Indonesian compliance
- ✅ License workflow status management
- ✅ Error fixes untuk backend implementation

### 📝 Deliverable untuk Completion

#### 1. Frontend License Dashboard & Listing (Prioritas Tinggi)

- [ ] `/app/licenses/page.tsx` - License listing dengan status
- [ ] `/components/ui/licenses/LicenseCard.tsx` - Card component
- [ ] `/components/ui/licenses/StatusBadge.tsx` - Status visualization
- [ ] `/hooks/useLicenses.ts` - API integration untuk fetching licenses
- [ ] Implementasi filtering dan sorting

#### 2. License Application Form (Prioritas Tinggi)

- [ ] `/app/licenses/new/page.tsx` - Multi-step form
- [ ] `/components/ui/licenses/forms/LicenseTypeSelector.tsx`
- [ ] `/components/ui/licenses/forms/CompanySelector.tsx`
- [ ] `/components/ui/licenses/forms/LicenseDetailsForm.tsx`
- [ ] `/components/ui/licenses/forms/ReviewSubmit.tsx`
- [ ] Form validasi dengan error handling

#### 3. License Detail Page (Prioritas Tinggi)

- [ ] `/app/licenses/[id]/page.tsx` - License detail & tracking
- [ ] `/components/ui/licenses/StatusTimeline.tsx` - Status history
- [ ] `/components/ui/licenses/ActionButton.tsx` - Submit/Cancel actions
- [ ] `/components/ui/licenses/DocumentList.tsx` - Document management

#### 4. Document Upload System (Prioritas Tinggi)

- [ ] Implementasi multipart form di backend handler
- [ ] File storage management dengan validasi
- [ ] `/components/ui/documents/FileUploader.tsx` - Frontend component
- [ ] `/components/ui/documents/DocumentPreview.tsx` - Preview component

#### 5. Admin License Management UI (Prioritas Medium)

- [ ] `/app/admin/licenses/page.tsx` - Admin license listing
- [ ] `/app/admin/licenses/[id]/page.tsx` - Admin review page
- [ ] `/components/ui/admin/ApprovalForm.tsx` - Approval UI
- [ ] `/components/ui/admin/RejectionForm.tsx` - Rejection UI

### 🛠️ Technical Implementation Steps

#### 1. Frontend API Client (2 days)

```typescript
// /lib/api/licenses.ts
import { api } from "@/lib/api";

export type License = {
  id: string;
  license_type: string;
  company_id: string;
  user_id: string;
  title: string;
  description: string;
  application_status: string;
  // other fields
};

export const getLicenses = async (): Promise<License[]> => {
  const response = await api.get("/licenses");
  return response.data;
};

export const getLicenseById = async (id: string): Promise<License> => {
  const response = await api.get(`/licenses/${id}`);
  return response.data;
};

export const createLicense = async (data: any): Promise<License> => {
  const response = await api.post("/licenses", data);
  return response.data;
};

export const submitLicense = async (id: string): Promise<License> => {
  const response = await api.post(`/licenses/${id}/submit`);
  return response.data;
};

// Additional methods...
```

#### 2. License Dashboard (3 days)

```tsx
// /app/licenses/page.tsx
"use client";

import { useEffect, useState } from "react";
import { getLicenses } from "@/lib/api/licenses";
import { LicenseCard } from "@/components/ui/licenses/LicenseCard";
import { Spinner } from "@/components/ui/common/Spinner";

export default function LicenseDashboard() {
  const [licenses, setLicenses] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchLicenses = async () => {
      try {
        const data = await getLicenses();
        setLicenses(data);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchLicenses();
  }, []);

  if (loading) return <Spinner />;
  if (error) return <div>Error loading licenses: {error}</div>;

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-6">My Licenses</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {licenses.length > 0 ? (
          licenses.map((license) => (
            <LicenseCard key={license.id} license={license} />
          ))
        ) : (
          <div className="col-span-full text-center py-10">
            <p>No licenses found. Start by applying for a new license.</p>
            <Link href="/licenses/new" className="btn btn-primary mt-4">
              Apply for License
            </Link>
          </div>
        )}
      </div>
    </div>
  );
}
```

#### 3. Document Upload Component (2 days)

```tsx
// /components/ui/documents/FileUploader.tsx
"use client";

import { useState } from "react";
import { uploadDocument } from "@/lib/api/licenses";

export function FileUploader({ licenseId, onUploadComplete }) {
  const [file, setFile] = useState(null);
  const [documentType, setDocumentType] = useState("");
  const [uploading, setUploading] = useState(false);
  const [error, setError] = useState("");

  const handleFileChange = (e) => {
    const selectedFile = e.target.files[0];
    if (selectedFile) {
      // Check file size and type
      if (selectedFile.size > 5 * 1024 * 1024) {
        setError("File too large. Maximum size is 5MB.");
        return;
      }
      setFile(selectedFile);
      setError("");
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!file || !documentType) {
      setError("Please select a file and document type");
      return;
    }

    setUploading(true);
    setError("");

    try {
      const formData = new FormData();
      formData.append("file", file);
      formData.append("documentType", documentType);

      await uploadDocument(licenseId, formData);
      setFile(null);
      setDocumentType("");
      onUploadComplete && onUploadComplete();
    } catch (err) {
      setError("Upload failed: " + err.message);
    } finally {
      setUploading(false);
    }
  };

  return (
    <div className="border border-gray-200 rounded-lg p-4">
      <h3 className="font-semibold mb-4">Upload Document</h3>

      {error && <div className="text-red-500 mb-4">{error}</div>}

      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">
            Document Type
          </label>
          <select
            value={documentType}
            onChange={(e) => setDocumentType(e.target.value)}
            className="w-full border rounded p-2"
            required
          >
            <option value="">Select document type</option>
            <option value="KTP">KTP (ID Card)</option>
            <option value="NPWP">NPWP (Tax ID)</option>
            <option value="BusinessDeed">Business Deed</option>
            <option value="BankStatement">Bank Statement</option>
            <option value="BusinessPlan">Business Plan</option>
            <option value="LocationPermit">Location Permit</option>
          </select>
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Select File</label>
          <input
            type="file"
            onChange={handleFileChange}
            className="w-full border rounded p-2"
            accept=".pdf,.jpg,.jpeg,.png"
          />
          <p className="text-xs text-gray-500 mt-1">
            Maximum file size: 5MB. Supported formats: PDF, JPG, PNG
          </p>
        </div>

        <button
          type="submit"
          disabled={uploading || !file || !documentType}
          className="btn btn-primary w-full"
        >
          {uploading ? "Uploading..." : "Upload Document"}
        </button>
      </form>
    </div>
  );
}
```

#### 4. Backend Document Upload Handler (2 days)

Implementasi handler yang sudah dirancang di PHASE-2-NEXT-STEPS.md:

```rust
// Implement in backend/src/infrastructure/web/handlers/licenses.rs

use axum::{
    extract::Multipart,
    response::IntoResponse,
};
use tower_http::services::ServeFile;

// Upload license document with multipart form
async fn upload_license_document(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<LicenseDocument>, StatusCode> {
    // Check license ownership first
    let license = match app_state.license_repository.get_license_by_id(license_id).await {
        Ok(Some(license)) => {
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Process file upload
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("unknown").to_string();
        let file_name = field.file_name().unwrap_or("unknown.pdf").to_string();
        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        // Validate file type and size
        // Save file to storage
        // Create document record

        let document = LicenseDocument {
            id: Uuid::new_v4(),
            license_id,
            document_type: determine_document_type(&name),
            file_name,
            file_path: format!("uploads/{}/{}", license_id, file_name),
            content_type,
            size_bytes: data.len() as i64,
            uploaded_at: Utc::now(),
            uploaded_by: *user.user_id.as_uuid(),
            verified: false,
        };

        match app_state.license_repository.create_document(&document).await {
            Ok(saved_doc) => return Ok(Json(saved_doc)),
            Err(e) => {
                tracing::error!("Failed to save document: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    Err(StatusCode::BAD_REQUEST)
}
```

### 📅 Timeline & Resource Allocation

| Task                     | Days | Priority | Dev Allocation |
| ------------------------ | ---- | -------- | -------------- |
| License API Client       | 2    | High     | Frontend Dev 1 |
| License Dashboard        | 3    | High     | Frontend Dev 1 |
| License Detail Page      | 2    | High     | Frontend Dev 2 |
| License Application Form | 3    | High     | Frontend Dev 2 |
| Document Upload Frontend | 2    | High     | Frontend Dev 1 |
| Document Upload Backend  | 2    | High     | Backend Dev    |
| Admin License UI         | 3    | Medium   | Frontend Dev 1 |
| Testing & Bugfixes       | 2    | High     | All Devs       |

### 🧪 Testing Plan

1. **Unit Tests**

   - Frontend components rendering
   - Backend handler functionality

2. **Integration Tests**

   - License application flow from start to finish
   - Document upload and verification
   - Status transitions

3. **User Acceptance Testing**
   - Complete license application as user
   - Process license as admin
   - View and manage licenses

### 🚀 Deployment Plan

1. Apply new database migrations (if any)
2. Deploy backend updates with new document handling
3. Deploy frontend updates with new license UI
4. Verify all functionality in staging
5. Conduct performance tests (especially for document upload)
6. Release to production

### 🎯 Success Metrics

1. License application completion rate > 90%
2. Document upload success rate > 95%
3. Admin processing time < 5 minutes per license
4. User satisfaction score > 4.5/5 for license management process
