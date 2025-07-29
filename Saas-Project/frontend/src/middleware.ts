import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { jwtVerify } from 'jose';

const secret = new TextEncoder().encode(process.env.JWT_SECRET || 'fallback-secret-key');

interface JWTPayload {
  id: string;
  email: string;
  role: string;
  exp: number;
}

export async function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Allow public routes
  const publicRoutes = [
    '/',
    '/umkm/login',
    '/umkm/register',
    '/auth/login',
    '/auth/register',
    '/api/auth/login',
    '/api/auth/register'
  ];

  if (publicRoutes.includes(pathname) || pathname.startsWith('/_next') || pathname.startsWith('/api/')) {
    return NextResponse.next();
  }

  // Get token from Authorization header or cookies
  const authHeader = request.headers.get('Authorization');
  const token = authHeader?.startsWith('Bearer ') 
    ? authHeader.substring(7) 
    : request.cookies.get('auth-token')?.value;

  if (!token) {
    // Redirect to appropriate login based on route
    if (pathname.startsWith('/umkm/')) {
      return NextResponse.redirect(new URL('/umkm/login', request.url));
    }
    return NextResponse.redirect(new URL('/auth/login', request.url));
  }

  try {
    // Verify the JWT token
    const { payload } = await jwtVerify(token, secret) as { payload: JWTPayload };
    
    // Role-based route protection
    if (pathname.startsWith('/umkm/')) {
      // UMKM routes - only accessible by umkm_owner
      if (payload.role !== 'umkm_owner') {
        return NextResponse.redirect(new URL('/auth/login', request.url));
      }
    } else if (pathname.startsWith('/staff/')) {
      // Staff routes - only accessible by admin_staff
      if (payload.role !== 'admin_staff') {
        return NextResponse.redirect(new URL('/auth/login', request.url));
      }
    } else if (pathname.startsWith('/dashboard') || pathname.startsWith('/admin') || pathname.startsWith('/companies')) {
      // SaaS admin routes - accessible by super_admin and admin_staff
      if (payload.role !== 'super_admin' && payload.role !== 'admin_staff') {
        return NextResponse.redirect(new URL('/umkm/login', request.url));
      }
    }

    // Add user info to request headers for downstream use
    const requestHeaders = new Headers(request.headers);
    requestHeaders.set('x-user-id', payload.id);
    requestHeaders.set('x-user-email', payload.email);
    requestHeaders.set('x-user-role', payload.role);

    return NextResponse.next({
      request: {
        headers: requestHeaders,
      },
    });

  } catch (error) {
    console.error('JWT verification failed:', error);
    
    // Clear invalid token
    const response = pathname.startsWith('/umkm/') 
      ? NextResponse.redirect(new URL('/umkm/login', request.url))
      : NextResponse.redirect(new URL('/auth/login', request.url));
    
    response.cookies.delete('auth-token');
    return response;
  }
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - api (API routes)
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     * - public folder
     */
    '/((?!api|_next/static|_next/image|favicon.ico|public).*)',
  ],
};
