///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// This software is supplied under the terms of a license agreement or
// nondisclosure agreement and may not be copied or disclosed except in
// accordance with the terms of that agreement.
//
// Copyright (c) 2009 Jesper Svennevid, Daniel Collin.
// All Rights Reserved.
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#ifndef zenic_Allocator_h
#define zenic_Allocator_h

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include <Shared/Core/Types.h>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct AllocatorRewindPoint
{
	uint32_t* currentPosition;
} AllocatorRewindPoint;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//! \brief Sets the start of the global memory to be used (On ps2 for example this may be at the _end label)
//! \param[in] start start address of the memory 

void Allocator_setStart(void* start, uint32_t totalSize, uint32_t alignment);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//! \brief "Allocates" memory from the availible memory.
//! Notice that this function doesn't really allocate memory but starts from a given memory adress and just moves
//! a pointer
//! \param[in] debugname 
//! \param[in] size of allocation 
//! \param[in] alignment alignment that the allocation needs 
//! \param[out] Pointer to the memory 

void* Allocator_alloc(const char* debugName, uint32_t size, uint32_t alignment);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//! \brief Makes it possible to get a rewindpoint to rewind the memory back to the memory position at the point
//! when the function was called (use Allocator_restoreRewindPoint) to rewind back.
//! Notice that "rewind" in this sense is just to move a pointer back. It actually doesn't do anything with the memory
//! \param[out] rewindpoint  

AllocatorRewindPoint Allocator_getRewindPoint();

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// \brief Rewinds the allocator to a given rewindpoint
//! \param[in] rewindPoint Rewindpoint to rewind back to 

void Allocator_rewind(const AllocatorRewindPoint* rewindPoint);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#endif

