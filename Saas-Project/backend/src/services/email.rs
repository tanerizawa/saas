use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EmailService {
    smtp_config: SmtpConfig,
    templates: HashMap<String, EmailTemplate>,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequest {
    pub to: String,
    pub to_name: Option<String>,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
    pub template_variables: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub enum EmailError {
    SmtpError(String),
    TemplateNotFound(String),
    InvalidAddress(String),
    SendFailed(String),
    ConfigurationError(String),
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::SmtpError(msg) => write!(f, "SMTP error: {}", msg),
            EmailError::TemplateNotFound(template) => write!(f, "Template not found: {}", template),
            EmailError::InvalidAddress(addr) => write!(f, "Invalid email address: {}", addr),
            EmailError::SendFailed(msg) => write!(f, "Failed to send email: {}", msg),
            EmailError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}

impl EmailService {
    pub fn new() -> Self {
        // Default configuration - will be loaded from environment
        let smtp_config = SmtpConfig {
            host: std::env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            username: std::env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string()),
            password: std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string()),
            from_address: std::env::var("SMTP_FROM_ADDRESS")
                .unwrap_or_else(|_| "noreply@saasumkm.id".to_string()),
            from_name: std::env::var("SMTP_FROM_NAME")
                .unwrap_or_else(|_| "SaaS UMKM Platform".to_string()),
        };

        let mut templates = HashMap::new();
        templates.insert("welcome".to_string(), Self::get_welcome_template());
        templates.insert("verification".to_string(), Self::get_verification_template());
        templates.insert("password_reset".to_string(), Self::get_password_reset_template());
        templates.insert("onboarding_next_step".to_string(), Self::get_onboarding_template());
        templates.insert("account_activated".to_string(), Self::get_account_activated_template());
        templates.insert("license_approved".to_string(), Self::get_license_approved_template());
        templates.insert("license_rejected".to_string(), Self::get_license_rejected_template());

        Self {
            smtp_config,
            templates,
        }
    }

    /// Send welcome email to new users
    pub async fn send_welcome_email(
        &self,
        email: &str,
        full_name: &str,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("platform_url".to_string(), self.get_platform_url());

        self.send_template_email(email, Some(full_name), "welcome", variables)
            .await
    }

    /// Send email verification
    pub async fn send_verification_email(
        &self,
        email: &str,
        full_name: &str,
        verification_token: &str,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("verification_link".to_string(), 
            format!("{}/auth/verify?token={}", self.get_platform_url(), verification_token)
        );

        self.send_template_email(email, Some(full_name), "verification", variables)
            .await
    }

    /// Send password reset email
    pub async fn send_password_reset_email(
        &self,
        email: &str,
        full_name: &str,
        reset_token: &str,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("reset_link".to_string(), 
            format!("{}/auth/reset-password?token={}", self.get_platform_url(), reset_token)
        );

        self.send_template_email(email, Some(full_name), "password_reset", variables)
            .await
    }

    /// Send onboarding next step email
    pub async fn send_onboarding_email(
        &self,
        email: &str,
        full_name: &str,
        next_step: &str,
        completion_percentage: u8,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("next_step".to_string(), next_step.to_string());
        variables.insert("completion_percentage".to_string(), completion_percentage.to_string());
        variables.insert("dashboard_url".to_string(), format!("{}/umkm/dashboard", self.get_platform_url()));

        self.send_template_email(email, Some(full_name), "onboarding_next_step", variables)
            .await
    }

    /// Send account activation confirmation
    pub async fn send_account_activated_email(
        &self,
        email: &str,
        full_name: &str,
        company_name: &str,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("company_name".to_string(), company_name.to_string());
        variables.insert("login_url".to_string(), format!("{}/umkm/login", self.get_platform_url()));

        self.send_template_email(email, Some(full_name), "account_activated", variables)
            .await
    }

    /// Send license approval notification
    pub async fn send_license_approved_email(
        &self,
        email: &str,
        full_name: &str,
        license_type: &str,
        license_number: &str,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("license_type".to_string(), license_type.to_string());
        variables.insert("license_number".to_string(), license_number.to_string());
        variables.insert("licenses_url".to_string(), format!("{}/umkm/licenses", self.get_platform_url()));

        self.send_template_email(email, Some(full_name), "license_approved", variables)
            .await
    }

    /// Send license rejection notification
    pub async fn send_license_rejected_email(
        &self,
        email: &str,
        full_name: &str,
        license_type: &str,
        rejection_reason: &str,
    ) -> Result<(), EmailError> {
        let mut variables = HashMap::new();
        variables.insert("full_name".to_string(), full_name.to_string());
        variables.insert("license_type".to_string(), license_type.to_string());
        variables.insert("rejection_reason".to_string(), rejection_reason.to_string());
        variables.insert("licenses_url".to_string(), format!("{}/umkm/licenses", self.get_platform_url()));

        self.send_template_email(email, Some(full_name), "license_rejected", variables)
            .await
    }

    /// Send email using template
    pub async fn send_template_email(
        &self,
        to: &str,
        to_name: Option<&str>,
        template_name: &str,
        variables: HashMap<String, String>,
    ) -> Result<(), EmailError> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| EmailError::TemplateNotFound(template_name.to_string()))?;

        let subject = self.replace_variables(&template.subject, &variables);
        let html_body = self.replace_variables(&template.html_body, &variables);
        let text_body = self.replace_variables(&template.text_body, &variables);

        let request = EmailRequest {
            to: to.to_string(),
            to_name: to_name.map(|s| s.to_string()),
            subject,
            html_body,
            text_body,
            template_variables: Some(variables),
        };

        self.send_email(request).await
    }

    /// Send email directly
    pub async fn send_email(&self, request: EmailRequest) -> Result<(), EmailError> {
        // TODO: Implement actual SMTP sending
        // For now, just log the email (in production, use a proper SMTP library like lettre)
        
        tracing::info!(
            "Sending email to: {} | Subject: {} | From: {}",
            request.to,
            request.subject,
            self.smtp_config.from_address
        );

        // In development, we might want to save emails to files or use a test SMTP server
        if cfg!(debug_assertions) {
            self.log_email_for_development(&request).await?;
        } else {
            self.send_via_smtp(&request).await?;
        }

        Ok(())
    }

    // Private helper methods

    fn get_platform_url(&self) -> String {
        std::env::var("PLATFORM_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    }

    fn replace_variables(&self, template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    async fn log_email_for_development(&self, request: &EmailRequest) -> Result<(), EmailError> {
        // In development, save email to file for testing
        let email_content = format!(
            "To: {}\nSubject: {}\n\n{}\n\n---\n{}",
            request.to, request.subject, request.text_body, request.html_body
        );

        // Create emails directory if it doesn't exist
        tokio::fs::create_dir_all("./dev_emails").await
            .map_err(|e| EmailError::SendFailed(format!("Failed to create dev_emails directory: {}", e)))?;

        // Save email to file
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("./dev_emails/email_{}_{}.txt", timestamp, request.to.replace("@", "_at_"));
        
        tokio::fs::write(&filename, email_content).await
            .map_err(|e| EmailError::SendFailed(format!("Failed to save email to file: {}", e)))?;

        tracing::info!("Development email saved to: {}", filename);
        Ok(())
    }

    async fn send_via_smtp(&self, _request: &EmailRequest) -> Result<(), EmailError> {
        // TODO: Implement actual SMTP sending using lettre or similar
        // This would involve:
        // 1. Creating SMTP client
        // 2. Building email message
        // 3. Sending via configured SMTP server
        
        tracing::warn!("SMTP sending not implemented yet - email would be sent in production");
        Ok(())
    }

    // Email templates

    fn get_welcome_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Selamat Datang di SaaS UMKM Platform!".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #2563eb; text-align: center;">Selamat Datang!</h1>
                        <p>Halo {full_name},</p>
                        <p>Terima kasih telah bergabung dengan SaaS UMKM Platform. Kami sangat senang dapat membantu Anda mengelola bisnis UMKM dengan lebih efisien.</p>
                        <div style="background-color: #f8fafc; padding: 20px; border-radius: 5px; margin: 20px 0;">
                            <h3>Langkah Selanjutnya:</h3>
                            <ul>
                                <li>Verifikasi alamat email Anda</li>
                                <li>Lengkapi profil perusahaan</li>
                                <li>Upload dokumen yang diperlukan</li>
                                <li>Mulai menggunakan platform</li>
                            </ul>
                        </div>
                        <p style="text-align: center;">
                            <a href="{platform_url}" style="background-color: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Mulai Sekarang</a>
                        </p>
                        <p style="color: #666; font-size: 14px; text-align: center; margin-top: 30px;">
                            Tim SaaS UMKM<br>
                            <a href="{platform_url}">www.saasumkm.id</a>
                        </p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Selamat Datang di SaaS UMKM Platform!

                Halo {full_name},

                Terima kasih telah bergabung dengan SaaS UMKM Platform. Kami sangat senang dapat membantu Anda mengelola bisnis UMKM dengan lebih efisien.

                Langkah Selanjutnya:
                - Verifikasi alamat email Anda
                - Lengkapi profil perusahaan
                - Upload dokumen yang diperlukan
                - Mulai menggunakan platform

                Kunjungi: {platform_url}

                Tim SaaS UMKM
                www.saasumkm.id
            "#.to_string(),
        }
    }

    fn get_verification_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Verifikasi Email Anda - SaaS UMKM".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #2563eb; text-align: center;">Verifikasi Email</h1>
                        <p>Halo {full_name},</p>
                        <p>Silakan klik tombol di bawah ini untuk memverifikasi alamat email Anda:</p>
                        <p style="text-align: center; margin: 30px 0;">
                            <a href="{verification_link}" style="background-color: #16a34a; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Verifikasi Email</a>
                        </p>
                        <p style="color: #666; font-size: 14px;">
                            Jika Anda tidak dapat mengklik tombol di atas, salin dan tempel link berikut ke browser Anda:<br>
                            {verification_link}
                        </p>
                        <p style="color: #666; font-size: 14px;">Link ini akan kedaluwarsa dalam 24 jam.</p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Verifikasi Email - SaaS UMKM

                Halo {full_name},

                Silakan kunjungi link berikut untuk memverifikasi alamat email Anda:
                {verification_link}

                Link ini akan kedaluwarsa dalam 24 jam.

                Tim SaaS UMKM
            "#.to_string(),
        }
    }

    fn get_password_reset_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Reset Password - SaaS UMKM".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #2563eb; text-align: center;">Reset Password</h1>
                        <p>Halo {full_name},</p>
                        <p>Kami menerima permintaan untuk mereset password akun Anda. Klik tombol di bawah ini untuk membuat password baru:</p>
                        <p style="text-align: center; margin: 30px 0;">
                            <a href="{reset_link}" style="background-color: #dc2626; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Reset Password</a>
                        </p>
                        <p style="color: #666; font-size: 14px;">
                            Jika Anda tidak meminta reset password, abaikan email ini. Password Anda tidak akan berubah.
                        </p>
                        <p style="color: #666; font-size: 14px;">Link ini akan kedaluwarsa dalam 1 jam.</p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Reset Password - SaaS UMKM

                Halo {full_name},

                Kami menerima permintaan untuk mereset password akun Anda. Kunjungi link berikut untuk membuat password baru:
                {reset_link}

                Jika Anda tidak meminta reset password, abaikan email ini.
                Link ini akan kedaluwarsa dalam 1 jam.

                Tim SaaS UMKM
            "#.to_string(),
        }
    }

    fn get_onboarding_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Langkah Selanjutnya Onboarding - SaaS UMKM".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #2563eb; text-align: center;">Proses Onboarding</h1>
                        <p>Halo {full_name},</p>
                        <p>Anda telah menyelesaikan {completion_percentage}% dari proses onboarding!</p>
                        <div style="background-color: #f0f9ff; padding: 20px; border-radius: 5px; margin: 20px 0;">
                            <h3>Langkah Selanjutnya:</h3>
                            <p style="font-size: 16px; color: #1e40af;">{next_step}</p>
                        </div>
                        <p style="text-align: center;">
                            <a href="{dashboard_url}" style="background-color: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Lanjutkan Onboarding</a>
                        </p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Langkah Selanjutnya Onboarding - SaaS UMKM

                Halo {full_name},

                Anda telah menyelesaikan {completion_percentage}% dari proses onboarding!

                Langkah Selanjutnya: {next_step}

                Lanjutkan di: {dashboard_url}

                Tim SaaS UMKM
            "#.to_string(),
        }
    }

    fn get_account_activated_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Akun Anda Telah Diaktifkan! - SaaS UMKM".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #16a34a; text-align: center;">🎉 Akun Diaktifkan!</h1>
                        <p>Halo {full_name},</p>
                        <p>Selamat! Akun perusahaan <strong>{company_name}</strong> telah berhasil diaktifkan.</p>
                        <p>Anda sekarang dapat:</p>
                        <ul>
                            <li>Mengakses semua fitur platform</li>
                            <li>Mengajukan berbagai jenis lisensi</li>
                            <li>Mengelola dokumen perusahaan</li>
                            <li>Membuat laporan keuangan</li>
                        </ul>
                        <p style="text-align: center; margin: 30px 0;">
                            <a href="{login_url}" style="background-color: #16a34a; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Masuk ke Platform</a>
                        </p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Akun Anda Telah Diaktifkan! - SaaS UMKM

                Halo {full_name},

                Selamat! Akun perusahaan {company_name} telah berhasil diaktifkan.

                Anda sekarang dapat mengakses semua fitur platform.

                Masuk di: {login_url}

                Tim SaaS UMKM
            "#.to_string(),
        }
    }

    fn get_license_approved_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Lisensi Anda Disetujui! - SaaS UMKM".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #16a34a; text-align: center;">✅ Lisensi Disetujui!</h1>
                        <p>Halo {full_name},</p>
                        <p>Selamat! Pengajuan lisensi <strong>{license_type}</strong> Anda telah disetujui.</p>
                        <div style="background-color: #f0fdf4; padding: 20px; border-radius: 5px; margin: 20px 0;">
                            <p><strong>Nomor Lisensi:</strong> {license_number}</p>
                            <p><strong>Jenis Lisensi:</strong> {license_type}</p>
                        </div>
                        <p style="text-align: center;">
                            <a href="{licenses_url}" style="background-color: #16a34a; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Lihat Detail Lisensi</a>
                        </p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Lisensi Anda Disetujui! - SaaS UMKM

                Halo {full_name},

                Selamat! Pengajuan lisensi {license_type} Anda telah disetujui.

                Nomor Lisensi: {license_number}

                Lihat detail di: {licenses_url}

                Tim SaaS UMKM
            "#.to_string(),
        }
    }

    fn get_license_rejected_template() -> EmailTemplate {
        EmailTemplate {
            subject: "Pengajuan Lisensi Perlu Diperbaiki - SaaS UMKM".to_string(),
            html_body: r#"
                <html>
                <body style="font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
                    <div style="max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 10px;">
                        <h1 style="color: #dc2626; text-align: center;">📋 Perbaikan Diperlukan</h1>
                        <p>Halo {full_name},</p>
                        <p>Pengajuan lisensi <strong>{license_type}</strong> Anda memerlukan perbaikan.</p>
                        <div style="background-color: #fef2f2; padding: 20px; border-radius: 5px; margin: 20px 0;">
                            <h3>Alasan:</h3>
                            <p>{rejection_reason}</p>
                        </div>
                        <p>Silakan perbaiki dokumen atau informasi yang diperlukan, lalu ajukan kembali.</p>
                        <p style="text-align: center;">
                            <a href="{licenses_url}" style="background-color: #dc2626; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Perbaiki Pengajuan</a>
                        </p>
                    </div>
                </body>
                </html>
            "#.to_string(),
            text_body: r#"
                Pengajuan Lisensi Perlu Diperbaiki - SaaS UMKM

                Halo {full_name},

                Pengajuan lisensi {license_type} Anda memerlukan perbaikan.

                Alasan: {rejection_reason}

                Silakan perbaiki di: {licenses_url}

                Tim SaaS UMKM
            "#.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_service_creation() {
        let service = EmailService::new();
        assert!(!service.templates.is_empty());
        assert!(service.templates.contains_key("welcome"));
        assert!(service.templates.contains_key("verification"));
    }

    #[test]
    fn test_variable_replacement() {
        let service = EmailService::new();
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("platform".to_string(), "SaaS UMKM".to_string());

        let template = "Hello {name}, welcome to {platform}!";
        let result = service.replace_variables(template, &variables);
        assert_eq!(result, "Hello John, welcome to SaaS UMKM!");
    }
}
