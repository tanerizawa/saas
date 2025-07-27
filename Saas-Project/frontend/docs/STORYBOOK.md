# Storybook Guide

## Pendahuluan

Storybook adalah alat untuk mengembangkan UI components secara terisolasi. Hal ini membantu:
- Membangun komponen secara terisolasi dari bisnis logika dan konteks aplikasi
- Mendokumentasikan komponen untuk tim
- Test visual untuk mencegah regresi tampilan
- Memastikan aksesibilitas dan standardisasi UI

## Cara Menggunakan Storybook

### Menjalankan Storybook

```bash
npm run storybook
```

Ini akan membuka browser dengan Storybook pada http://localhost:6006 (atau port lain jika 6006 sudah digunakan).

### Struktur Story

Setiap story file disimpan di direktori `src/stories` dengan format `[NamaKomponen].stories.tsx`. Format dasar story:

```typescript
import type { Meta, StoryObj } from '@storybook/react';
import { Button } from '../components/ui/button';

const meta: Meta<typeof Button> = {
  title: 'Components/UI/Button',  // Kategori/nama komponen
  component: Button,
  tags: ['autodocs'],  // Generate dokumentasi otomatis
  parameters: {
    layout: 'centered',  // Posisi komponen di viewport
  },
  argTypes: {
    // Mendefinisikan argumen kontrol untuk komponen
    variant: {
      control: { type: 'select' },
      options: ['default', 'destructive', 'outline', 'secondary'],
    },
  },
};

export default meta;
type Story = StoryObj<typeof Button>;

// Definisi story
export const Default: Story = {
  args: {
    children: 'Button',
    variant: 'default',
  },
};

// Story lain dengan variasi berbeda
export const Destructive: Story = {
  args: {
    children: 'Delete',
    variant: 'destructive',
  },
};
```

## Praktik Terbaik

### Penulisan Stories

1. **Buat Story Untuk Setiap Variasi Penting**
   - Standar states: default, hover, fokus, disabled
   - Variasi sesuai props: ukuran, warna, layout options

2. **Gunakan Args Untuk Parameter**
   - Gunakan `args` untuk memungkinkan penyesuaian komponen dari UI
   - Definisikan `argTypes` untuk kontrol yang lebih baik

3. **Kelompokkan Stories Berdasarkan Kategori**
   - Gunakan format `title: 'Category/Subcategory/ComponentName'`
   - Contoh: `UI/Form/Button` atau `Pages/Authentication/LoginForm`

4. **Tambahkan Dokumentasi**
   - Gunakan JSDoc atau komentar pada komponen
   - Aktifkan `autodocs` untuk dokumentasi otomatis

### Testing Aksesibilitas

Storybook memiliki addon untuk A11y testing:

```typescript
// .storybook/preview.js
export const parameters = {
  a11y: {
    // options: https://github.com/dequelabs/axe-core/blob/develop/doc/API.md#options-parameter
    options: {},
    // optionally pass manual configurations
    config: { rules: [] },
    // optional selector to inspect
    element: '#root',
  },
};
```

## Integrasi dengan Vitest

Storybook terintegrasi dengan Vitest untuk testing. Gunakan addon `@storybook/addon-vitest` untuk mengakses stories dalam testing:

```typescript
// Component.test.tsx
import { composeStories } from '@storybook/react';
import { render, screen } from '@testing-library/react';
import * as stories from './Component.stories';

// Compose all stories from the import
const { Default, WithError } = composeStories(stories);

test('renders default state', () => {
  render(<Default />);
  expect(screen.getByRole('button')).toBeInTheDocument();
});
```

## Tips Khusus untuk Project Ini

1. **Komponen UI Dasar**
   - Semua komponen UI dasar harus memiliki story
   - Uji pada mode terang dan gelap menggunakan backgrounds addon

2. **Komponen Form**
   - Tampilkan states validasi (sukses, error, loading)
   - Gunakan mock handlers untuk mendemonstrasikan interaksi

3. **Components Layout**
   - Gunakan viewport addon untuk menguji responsivitas
   - Tunjukkan pada ukuran layar berbeda

4. **Dokumentasi Component API**
   - Jelaskan semua props dan behaviors
   - Berikan contoh kode penggunaan
