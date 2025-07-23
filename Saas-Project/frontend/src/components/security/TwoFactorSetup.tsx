"use client";

import { useState } from "react";

type TwoFactorSetupProps = {
  onSetupComplete?: () => void;
};

const TwoFactorSetup = ({ onSetupComplete }: TwoFactorSetupProps) => {
  const [step, setStep] = useState<"intro" | "qrcode" | "verify" | "backup">(
    "intro"
  );
  const [verificationCode, setVerificationCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [qrCodeUrl, setQrCodeUrl] = useState(""); // Would be set from API
  const [backupCodes, setBackupCodes] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  // Simulate QR code generation
  const generateQRCode = async () => {
    setLoading(true);

    try {
      // In a real implementation, this would be an API call
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // Mock QR code URL
      setQrCodeUrl(
        "https://api.qrserver.com/v1/create-qr-code/?data=otpauth://totp/SAAS-UMKM:user@example.com?secret=JBSWY3DPEHPK3PXP&issuer=SAAS-UMKM&algorithm=SHA1&digits=6&period=30"
      );

      setStep("qrcode");
    } catch (err) {
      setError("Failed to generate QR code");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // Verify entered code
  const verifyCode = async () => {
    if (verificationCode.length !== 6) {
      setError("Kode verifikasi harus 6 digit");
      return;
    }

    setLoading(true);

    try {
      // In a real implementation, this would be an API call
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // For demo, assume verification is successful if code is '123456'
      if (verificationCode === "123456") {
        // Generate backup codes
        const codes = Array.from(
          { length: 8 },
          () =>
            Math.random().toString(36).substring(2, 8) +
            Math.random().toString(36).substring(2, 8)
        );

        setBackupCodes(codes);
        setStep("backup");
        setError(null);
      } else {
        setError("Kode verifikasi tidak valid");
      }
    } catch (err) {
      setError("Gagal memverifikasi kode");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // Complete setup
  const completeSetup = () => {
    if (onSetupComplete) {
      onSetupComplete();
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-sm p-6 max-w-md mx-auto">
      <h2 className="text-2xl font-bold mb-4">
        Aktivasi Autentikasi Dua Faktor
      </h2>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      {step === "intro" && (
        <div>
          <p className="text-gray-600 mb-4">
            Autentikasi dua faktor menambahkan lapisan keamanan tambahan untuk
            akun Anda. Setiap kali login, Anda akan memasukkan kode dari
            aplikasi autentikator.
          </p>

          <div className="bg-blue-50 border-l-4 border-blue-500 p-4 mb-6">
            <div className="flex">
              <div className="flex-shrink-0">
                <svg
                  className="h-5 w-5 text-blue-400"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fillRule="evenodd"
                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2h-1V9z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <div className="ml-3">
                <p className="text-sm text-blue-700">
                  Anda memerlukan aplikasi autentikator seperti Google
                  Authenticator, Authy, atau Microsoft Authenticator.
                </p>
              </div>
            </div>
          </div>

          <button
            onClick={generateQRCode}
            disabled={loading}
            className={`w-full py-2 px-4 border border-transparent rounded-md shadow-sm text-white ${
              loading
                ? "bg-gray-400 cursor-not-allowed"
                : "bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            }`}
          >
            {loading ? "Mempersiapkan..." : "Mulai Setup"}
          </button>
        </div>
      )}

      {step === "qrcode" && (
        <div>
          <p className="text-gray-600 mb-4">
            Pindai kode QR ini dengan aplikasi autentikator Anda:
          </p>

          <div className="flex justify-center my-6">
            <img
              src={qrCodeUrl}
              alt="QR Code untuk autentikasi dua faktor"
              className="border border-gray-200 rounded-md p-2"
              width="200"
              height="200"
            />
          </div>

          <div className="mb-6">
            <label
              htmlFor="verification-code"
              className="block text-sm font-medium text-gray-700"
            >
              Masukkan kode verifikasi dari aplikasi Anda
            </label>
            <input
              type="text"
              id="verification-code"
              value={verificationCode}
              onChange={(e) =>
                setVerificationCode(
                  e.target.value.replace(/\D/g, "").slice(0, 6)
                )
              }
              className="mt-1 block w-full py-2 px-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              placeholder="6-digit code"
              maxLength={6}
              autoComplete="off"
            />
            <p className="mt-1 text-sm text-gray-500">
              Untuk demo, gunakan kode: 123456
            </p>
          </div>

          <div className="flex space-x-3">
            <button
              onClick={() => setStep("intro")}
              className="flex-1 py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Kembali
            </button>
            <button
              onClick={verifyCode}
              disabled={loading || verificationCode.length !== 6}
              className={`flex-1 py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white ${
                loading || verificationCode.length !== 6
                  ? "bg-gray-400 cursor-not-allowed"
                  : "bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              }`}
            >
              {loading ? "Memverifikasi..." : "Verifikasi"}
            </button>
          </div>
        </div>
      )}

      {step === "backup" && (
        <div>
          <p className="text-gray-600 mb-4">
            Autentikasi dua faktor telah diaktifkan! Simpan kode cadangan
            berikut di tempat yang aman:
          </p>

          <div className="bg-yellow-50 border-l-4 border-yellow-500 p-4 mb-4">
            <div className="flex">
              <div className="flex-shrink-0">
                <svg
                  className="h-5 w-5 text-yellow-400"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fillRule="evenodd"
                    d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <div className="ml-3">
                <p className="text-sm text-yellow-700">
                  Simpan kode cadangan ini. Anda akan membutuhkannya jika
                  kehilangan akses ke aplikasi autentikator Anda.
                </p>
              </div>
            </div>
          </div>

          <div className="bg-gray-50 p-4 rounded-md mb-6">
            <div className="grid grid-cols-2 gap-2">
              {backupCodes.map((code, index) => (
                <div
                  key={index}
                  className="font-mono text-sm border border-gray-200 rounded p-2 bg-white"
                >
                  {code}
                </div>
              ))}
            </div>
          </div>

          <button
            onClick={completeSetup}
            className="w-full py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500"
          >
            Selesai
          </button>
        </div>
      )}
    </div>
  );
};

export default TwoFactorSetup;
