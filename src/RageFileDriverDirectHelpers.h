/* RageFileDriverDirectHelpers - Internal helpers for RageFileDriverDirect. */

#ifndef RAGE_FILE_DRIVER_DIRECT_HELPERS_H
#define RAGE_FILE_DRIVER_DIRECT_HELPERS_H

#if defined(HAVE_FCNTL_H)
#include <fcntl.h>
#endif

#define DoStat stat
#define DoMkdir mkdir
#define DoFindFirstFile FindFirstFile
#define DoRename rename
#define DoRemove remove
#if defined(WIN32)
#define DoOpen _open
#define DoRmdir _rmdir
#define DoLseek _lseek
#define DoClose _close
#define DoRead _read
#define DoWrite _write
#define DoGetCwd _getcwd
#else
#define DoOpen open
#define DoRmdir rmdir
#define DoLseek lseek
#define DoClose close
#define DoRead read
#define DoWrite write
#define DoGetCwd getcwd
#endif
RString DoPathReplace( const RString &sPath );

#if defined(WIN32)
bool WinMoveFile( RString sOldPath, RString sNewPath );
#endif

#if !defined(O_BINARY)
#define O_BINARY 0
#endif

bool CreateDirectories( RString sPath );

#include "RageUtil_FileDB.h"
class DirectFilenameDB: public FilenameDB
{
public:
	DirectFilenameDB( RString root );
	void SetRoot( RString root );
	void CacheFile( const RString &sPath );
protected:
	virtual void PopulateFileSet( FileSet &fs, const RString &sPath );
	RString root;
};

#endif

/*
 * Copyright (c) 2003-2004 Glenn Maynard, Chris Danford
 * All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, and/or sell copies of the Software, and to permit persons to
 * whom the Software is furnished to do so, provided that the above
 * copyright notice(s) and this permission notice appear in all copies of
 * the Software and that both the above copyright notice(s) and this
 * permission notice appear in supporting documentation.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
 * THIRD PARTY RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS
 * INCLUDED IN THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT
 * OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
 * OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
