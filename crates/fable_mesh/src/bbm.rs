pub mod decode;
pub mod encode;

// Temporary comments from fabletlcmod.com.
//
//  Tagged Model Format
//
//  3DMF: 3D Mesh File
//      3DRT: (File Size of all Chunks)
//      MTLS: Material List (File Size of All MTRL Chunks)
//          MTRL: Material Description
//          MTLE: Extended Material Properties
//          MMAP: Mapping Info
//      SUBM: Sub-Mesh
//          TRFM: Transformation Matrix
//          PRIM: Primitive
//              TRIS: Triangle List
//              SMTH: Smoothing Groups
//              VERT: Vertex List
//              UNIV: Unique Vertex Information
//              VGRP: Vertex Group
//          BONE: Bone
//          CLTH: Cloth
//      HLPR: Helpers
//          HDMY: Dummy Object
//          HPNT: Helper Point
//          HCVL: Convex Volume
//
//  Compiled Model Format
//

pub struct Bbm {
    pub header: BbmHeader,
}


pub struct BbmHeader {
    pub unknown1: String,                   // char         NullTerminatedString[x];
    pub selection_present: u8,              // byte         SkeletonPresent;
    pub unknown2: Vec<f32>,                 // float        floats[10]; //Model Origin?? Listed in .big Sub-header...
    pub hpnt_count: u16,                    // word         HPNT_Count;
    pub hdmy_count: u16,                    // word         HDMY_Count;
    pub hlpr_index_uncompressed: u32,       // dword        HLPR_Index_Uncompressed;
    pub padding: u16,                       // word			padding;
    pub hpnt_compressed: u16,               // word			HPNT_Compressed;
    pub helper_points: Vec<BbmHelperPoint>,        // Helper Points[HPNT_Count];
    pub hdmy_compressed: u16,               // word			HDMY_Compressed;
    pub helper_dummies: Vec<BbmHelperDummy>,     // Helper Dummies[HDMY_Count];
    pub hlpr_index_compressed: u16,         // word			HLPR_Index_Compressed;
    pub hpnt_index_size: u16,               // word			HPNT_IndexSize;
    pub hpnt_index: Vec<u8>,                // char			HPNT_Index[HPNT_IndexSize-2]; //Subtract the size
    pub hdmy_index: Vec<u8>,                // char		    HDMY_Index[HLPR_Index_Uncompressed-HPNT_IndexSize]; //Rest of helper index deduced
    pub material_count: u32,                // dword        NumberMaterials;
    pub surface_count: u32,                 // dword        NumberSurfaces;
    pub bone_count: u32,                    // dword        NumberBones;
    pub bone_index_size: u32,               // dword        SizeOfBoneIndex;
    pub unknown3: u16,                      // byte         Unknown;
    pub unknown4: u16,                      // word         Unknown;
    pub unknown5: u16,                      // word         Unknown;
    pub compressed: u16,                    // word         Compressed;
    pub bone_index_reference: Vec<u16>,     // word         Bone_Index_Reference[NumberBones-1];
    pub bone_index_compressed: u16,         // word         BoneIndexCompressed;
    pub bone_index: Vec<u8>,                // char         BoneIndex[SizeOfBoneIndex];
    pub compressed_size: u16,               // word         CompressedSize;
    //      Bones SUB CHUNK 1[NumberBones];
    //      word					CompressedSize;
    //      Bones SUB CHUNK 2[NumberBones];
    //      word					CompressedSize;
    //      Bones SUB CHUNK 3[NumberBones];
    //      float					Matrix[12]; //Transform Matrix
}

pub struct BbmHelperPoint {
    pub matrix: Vec<f32>,   // float         Matrix[4]; //No Rotation
    pub hierarchy: i32,      // long          hierarchy;
}

pub struct BbmHelperDummy {
    // Why is this 13? No other matrix except 1x13 or 13x1 makes sense, unless its a 13x13 matrix? It could be, as Wikipedia puts, "alternative expressions of transformation matrices involving row vectors that are preferred by some authors." where 13x1 makes sense: https://en.wikipedia.org/wiki/Row_and_column_vectors
    pub matrix: Vec<f32>,   //  float        Matrix[13];
    pub hierarchy: i32,      //  long         hierarchy;
}

//
//  Bones
//      SUB CHUNK 1
//          long					Index;
//          long					Parent;
//          long					nChild;
//          float					Matrix[12];
//      SUB CHUNK 2
//          float					Matrix[12];
//      SUB CHUNK 3
//          float					Matrix[16];
//
//  Material List
//      dword					Material_Index;
//      char					NullTerminatedString[x];
//      dword					Padding;
//      dword					BASE_Texture_ID; //From Texture.big
//      dword					BUMPMAP_Texture_ID; //From Texture.big
//      dword					REFLECT_Texture_ID; //From Texture.big
//      dword					Unknown;
//      dword					Max_Texture_Layers;
//      dword					Glow_Strength;
//      byte					Unknown;
//      byte					Alpha_Enabled;
//      byte					Unknown;
//      word					Ignored; //For degenerate Tri's
//
//  Sub-Mesh
//  dword							Hierarchy;
//  dword							DestroyableMeshLevels;
//  float							floats[5];
//  dword							nFaceVertexIndices;
//  dword							nFaceVertexIndices_BoneIndice;
//  dword							nVerts;
//  dword							nFaces;
//  dword							nSourceVerts;
//  dword							Unknown;
//  dword							Unknown;
//  dword							Unknown;
//  struct structMTRLnFaceVertexIndices			FaceIndices[nFaceVertexIndices];
//  struct structMTRLnFaceVertexIndicesBoneIndice		Face_BoneIndices[nFaceVertexIndices_BoneIndice];
//  float							floats[8];
//  dword							sVert;
//  dword							padding;
//  //Start of Mesh
//
//  Quick notes on sVert “Size Vertice Blocks”….
//
//  20 - 28byte float coords, bones, packed normals, s11e4 tu tv
//  6 - 28byte packed coords, bones, packed normals, s11e4 tu, tv, dword[2]
//  4 - 36byte float coords, float normals, float tu tv, dword meshlevel
//  22 - 36byte float coords, bones, packed normals, s11e4 tu,tv, dword[2]
//  4 - 12byte packed coords, packed normals, s11e4 tu tv
//  4 - 20byte packed coords, bones, packed normals, s11e4 tu tv
//  20 - 20byte float coords, packed normals, s11e4 tu tv
//  Dynamic Clothing
//
//   struct CLTH
//  {
//  	//DWORD				SubMesh_ClothCount;
//  	DWORD				Cloth_ID;
//  	DWORD				??_ID; //possibly material ID
//  	DWORD				sChunk; //Size of full clothing data
//  	DWORD				Unknown5;
//  	DWORD				sDistanceIndice;
//  	CLTH_Distance*			DistanceIndice;//[sDistanceIndice/28]; //Distance between two particles
//  	float				Unknown8;
//  	float				Unknown9;
//  	float				Unknown10;
//  	DWORD				sParticleIndice;
//  	CLTH_Particle*			ParticleIndice;//[sParticleIndice];
//  	float*				ParticleAlphaIndice;//[sParticleIndice]; //How "free" they are. 0.0 = Static and gets a duped //  entry in verts
//  	DWORD				Unknown11;
//  	float				WindStrength; //strength
//  	char				EnableDragging; //enable
//  	char				RotationalDragging; //rotational
//  	float				StrengthDragging; //strength
//  	char				EnableAcceleration; //enable
//  	float				AccelerationDampening; //damping
//  	DWORD				nTriIndice;
//  	CLTH_Tri*			TriIndice;//[nTriIndice] Particles+"Unique" Verts
//  	DWORD				Unknown12; // looks like padding it
//  	DWORD				sTexIndice;
//  	CLTH_Tex*			TexIndice;//[sTexIndice]; //v1 = Particle/"unique" Vert, v2 = TexIndice
//  	DWORD				sVertexIndice;
//  	CLTH_Vertex*			VertexIndice;//[sVertexIndice];
//  	DWORD				sTexCoordIndice;
//  	CLTH_TexCoord*			TexCoordIndice;//[sTexCoordIndice];
//  	DWORD				sParticleMask;
//  	CLTH_PartMask*			ParticleMask;//[sParticleMask]; //Unique Particles in TriIndice
//  	DWORD				sVertMask;
//  	CLTH_VertMask*			Vertmask;//[sVertMask]; //Unique Verts in TriIndice
//  	//9 bytes of padding
//  	// 1 group for particles, 1 for verts
//  	DWORD				VGRPCount; // = Number of Bones
//  	VGRP**				VGRPs;
//  };