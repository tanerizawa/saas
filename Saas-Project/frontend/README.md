This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://nextjs.org/docs/app/api-reference/cli/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Storybook

This project uses [Storybook](https://storybook.js.org/) for component documentation and visual testing. To run Storybook:

```bash
npm run storybook
```

This will start Storybook on [http://localhost:6006](http://localhost:6006) (or another port if 6006 is in use).

### Component Stories

Component stories are located in the `src/stories` directory. Each story represents a component in different states or configurations. Stories help document components and enable visual testing.

We have implemented stories for the following components:
- Button - Basic action component with various styles and states
- Input - Form input fields with different types and states
- Select - Dropdown selection with single items and grouping
- Dialog - Modal dialogs for various use cases like confirmations and forms
- Tabs - Tabbed interface for content organization
- Toast - Notification messages with actions and various positioning
- Card - Content containers with headers, footers, and styling variations

### Adding New Stories

To create a new story for a component:

1. Create a new file in `src/stories` named `[ComponentName].stories.tsx`
2. Import your component and define stories for different states
3. Use the Storybook UI to view and test your component

For more detailed information on our Storybook implementation and best practices, see the [Storybook Guide](./docs/STORYBOOK.md).

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
