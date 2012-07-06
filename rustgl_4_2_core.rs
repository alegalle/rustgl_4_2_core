/************************************************************************
 * Opengl 4.2 core bindings for rust
 * 
 * This is a quick hack for playing around with opengl in rust
 * 
 * This is not an official opengl arb release and should not be 
 * treated as anything other than a library to tide you over until
 * an official release of gl3.h or similar for rust.
 * 
 * Only Opengl 4.2 core functions are included
 * 
 * You need to have a opengl 3.2 - 4.2 compliant GL library 
 * for this to compile
 * 
 * I have not included rust conversions for these functions as there
 * are so many so you will have to call the native functions direct
 * I also have not tested each and every one of the functions so there
 * may / will be errors in here somewhere
 * 
 * 
 *************************************************************************/


/*
** Copyright (c) 2007-2012 The Khronos Group Inc.
** 
** Permission is hereby granted, free of charge, to any person obtaining a
** copy of this software and/or associated documentation files (the
** "Materials"), to deal in the Materials without restriction, including
** without limitation the rights to use, copy, modify, merge, publish,
** distribute, sublicense, and/or sell copies of the Materials, and to
** permit persons to whom the Materials are furnished to do so, subject to
** the following conditions:
** 
** The above copyright notice and this permission notice shall be included
** in all copies or substantial portions of the Materials.
** 
** THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
** MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.
*/


/* Base GL types */

type GLenum = uint;
type GLboolean = u8;
type GLbitfield = uint;
type GLbyte = i8;
type GLshort = i16;
type GLint = int;
type GLsizei = int;
type GLubyte = u8;
type GLchar = u8;
type GLushort = u16;
type GLuint = uint;
type GLhalf = u16;
type GLfloat = f32;
type GLclampf = f32;
type GLdouble = f64;
type GLclampd = f64;
//type GLvoid = void;

type GLintptr = int;
type GLsizeiptr = int;

type GLint64 = i64;
type GLuint64 = u64;

type GLsync = *uint;


/*************************************************************/

const GL_DEPTH_BUFFER_BIT               : GLenum = 0x00000100u;
const GL_STENCIL_BUFFER_BIT             : GLenum = 0x00000400u;
const GL_COLOR_BUFFER_BIT               : GLenum = 0x00004000u;

const GL_FALSE                          : GLboolean = 0u8;
const GL_TRUE                           : GLboolean = 1u8;

const GL_POINTS                         : GLenum = 0x0000u;
const GL_LINES                          : GLenum = 0x0001u;
const GL_LINE_LOOP                      : GLenum = 0x0002u;
const GL_LINE_STRIP                     : GLenum = 0x0003u;
const GL_TRIANGLES                      : GLenum = 0x0004u;
const GL_TRIANGLE_STRIP                 : GLenum = 0x0005u;
const GL_TRIANGLE_FAN                   : GLenum = 0x0006u;

const GL_NEVER                          : GLenum = 0x0200u;
const GL_LESS                           : GLenum = 0x0201u;
const GL_EQUAL                          : GLenum = 0x0202u;
const GL_LEQUAL                         : GLenum = 0x0203u;
const GL_GREATER                        : GLenum = 0x0204u;
const GL_NOTEQUAL                       : GLenum = 0x0205u;
const GL_GEQUAL                         : GLenum = 0x0206u;
const GL_ALWAYS                         : GLenum = 0x0207u;

const GL_ZERO                           : GLboolean = 0u8;
const GL_ONE                            : GLboolean = 1u8;
const GL_SRC_COLOR                      : GLenum = 0x0300u;
const GL_ONE_MINUS_SRC_COLOR            : GLenum = 0x0301u;
const GL_SRC_ALPHA                      : GLenum = 0x0302u;
const GL_ONE_MINUS_SRC_ALPHA            : GLenum = 0x0303u;
const GL_DST_ALPHA                      : GLenum = 0x0304u;
const GL_ONE_MINUS_DST_ALPHA            : GLenum = 0x0305u;

const GL_DST_COLOR                      : GLenum = 0x0306u;
const GL_ONE_MINUS_DST_COLOR            : GLenum = 0x0307u;
const GL_SRC_ALPHA_SATURATE             : GLenum = 0x0308u;

const GL_NONE                           : GLboolean = 0u8;
const GL_FRONT_LEFT                     : GLenum = 0x0400u;
const GL_FRONT_RIGHT                    : GLenum = 0x0401u;
const GL_BACK_LEFT                      : GLenum = 0x0402u;
const GL_BACK_RIGHT                     : GLenum = 0x0403u;
const GL_FRONT                          : GLenum = 0x0404u;
const GL_BACK                           : GLenum = 0x0405u;
const GL_LEFT                           : GLenum = 0x0406u;
const GL_RIGHT                          : GLenum = 0x0407u;
const GL_FRONT_AND_BACK                 : GLenum = 0x0408u;

const GL_NO_ERROR                       : GLenum = 0u;
const GL_INVALID_ENUM                   : GLenum = 0x0500u;
const GL_INVALID_VALUE                  : GLenum = 0x0501u;
const GL_INVALID_OPERATION              : GLenum = 0x0502u;
const GL_OUT_OF_MEMORY                  : GLenum = 0x0505u;

const GL_CW                             : GLenum = 0x0900u;
const GL_CCW                            : GLenum = 0x0901u;

const GL_POINT_SIZE                     : GLenum = 0x0B11u;
const GL_POINT_SIZE_RANGE               : GLenum = 0x0B12u;
const GL_POINT_SIZE_GRANULARITY         : GLenum = 0x0B13u;
const GL_LINE_SMOOTH                    : GLenum = 0x0B20u;
const GL_LINE_WIDTH                     : GLenum = 0x0B21u;
const GL_LINE_WIDTH_RANGE               : GLenum = 0x0B22u;
const GL_LINE_WIDTH_GRANULARITY         : GLenum = 0x0B23u;
const GL_POLYGON_SMOOTH                 : GLenum = 0x0B41u;
const GL_CULL_FACE                      : GLenum = 0x0B44u;
const GL_CULL_FACE_MODE                 : GLenum = 0x0B45u;
const GL_FRONT_FACE                     : GLenum = 0x0B46u;
const GL_DEPTH_RANGE                    : GLenum = 0x0B70u;
const GL_DEPTH_TEST                     : GLenum = 0x0B71u;
const GL_DEPTH_WRITEMASK                : GLenum = 0x0B72u;
const GL_DEPTH_CLEAR_VALUE              : GLenum = 0x0B73u;
const GL_DEPTH_FUNC                     : GLenum = 0x0B74u;
const GL_STENCIL_TEST                   : GLenum = 0x0B90u;
const GL_STENCIL_CLEAR_VALUE            : GLenum = 0x0B91u;
const GL_STENCIL_FUNC                   : GLenum = 0x0B92u;
const GL_STENCIL_VALUE_MASK             : GLenum = 0x0B93u;
const GL_STENCIL_FAIL                   : GLenum = 0x0B94u;
const GL_STENCIL_PASS_DEPTH_FAIL        : GLenum = 0x0B95u;
const GL_STENCIL_PASS_DEPTH_PASS        : GLenum = 0x0B96u;
const GL_STENCIL_REF                    : GLenum = 0x0B97u;
const GL_STENCIL_WRITEMASK              : GLenum = 0x0B98u;
const GL_VIEWPORT                       : GLenum = 0x0BA2u;
const GL_DITHER                         : GLenum = 0x0BD0u;
const GL_BLEND_DST                      : GLenum = 0x0BE0u;
const GL_BLEND_SRC                      : GLenum = 0x0BE1u;
const GL_BLEND                          : GLenum = 0x0BE2u;
const GL_LOGIC_OP_MODE                  : GLenum = 0x0BF0u;
const GL_COLOR_LOGIC_OP                 : GLenum = 0x0BF2u;
const GL_DRAW_BUFFER                    : GLenum = 0x0C01u;
const GL_READ_BUFFER                    : GLenum = 0x0C02u;
const GL_SCISSOR_BOX                    : GLenum = 0x0C10u;
const GL_SCISSOR_TEST                   : GLenum = 0x0C11u;
const GL_COLOR_CLEAR_VALUE              : GLenum = 0x0C22u;
const GL_COLOR_WRITEMASK                : GLenum = 0x0C23u;
const GL_DOUBLEBUFFER                   : GLenum = 0x0C32u;
const GL_STEREO                         : GLenum = 0x0C33u;
const GL_LINE_SMOOTH_HINT               : GLenum = 0x0C52u;
const GL_POLYGON_SMOOTH_HINT            : GLenum = 0x0C53u;
const GL_UNPACK_SWAP_BYTES              : GLenum = 0x0CF0u;
const GL_UNPACK_LSB_FIRST               : GLenum = 0x0CF1u;
const GL_UNPACK_ROW_LENGTH              : GLenum = 0x0CF2u;
const GL_UNPACK_SKIP_ROWS               : GLenum = 0x0CF3u;
const GL_UNPACK_SKIP_PIXELS             : GLenum = 0x0CF4u;
const GL_UNPACK_ALIGNMENT               : GLenum = 0x0CF5u;
const GL_PACK_SWAP_BYTES                : GLenum = 0x0D00u;
const GL_PACK_LSB_FIRST                 : GLenum = 0x0D01u;
const GL_PACK_ROW_LENGTH                : GLenum = 0x0D02u;
const GL_PACK_SKIP_ROWS                 : GLenum = 0x0D03u;
const GL_PACK_SKIP_PIXELS               : GLenum = 0x0D04u;
const GL_PACK_ALIGNMENT                 : GLenum = 0x0D05u;
const GL_MAX_TEXTURE_SIZE               : GLenum = 0x0D33u;
const GL_MAX_VIEWPORT_DIMS              : GLenum = 0x0D3Au;
const GL_SUBPIXEL_BITS                  : GLenum = 0x0D50u;
const GL_TEXTURE_1D                     : GLenum = 0x0DE0u;
const GL_TEXTURE_2D                     : GLenum = 0x0DE1u;
const GL_POLYGON_OFFSET_UNITS           : GLenum = 0x2A00u;
const GL_POLYGON_OFFSET_POINT           : GLenum = 0x2A01u;
const GL_POLYGON_OFFSET_LINE            : GLenum = 0x2A02u;
const GL_POLYGON_OFFSET_FILL            : GLenum = 0x8037u;
const GL_POLYGON_OFFSET_FACTOR          : GLenum = 0x8038u;
const GL_TEXTURE_BINDING_1D             : GLenum = 0x8068u;
const GL_TEXTURE_BINDING_2D             : GLenum = 0x8069u;

const GL_TEXTURE_WIDTH                  : GLenum = 0x1000u;
const GL_TEXTURE_HEIGHT                 : GLenum = 0x1001u;
const GL_TEXTURE_INTERNAL_FORMAT        : GLenum = 0x1003u;
const GL_TEXTURE_BORDER_COLOR           : GLenum = 0x1004u;
const GL_TEXTURE_RED_SIZE               : GLenum = 0x805Cu;
const GL_TEXTURE_GREEN_SIZE             : GLenum = 0x805Du;
const GL_TEXTURE_BLUE_SIZE              : GLenum = 0x805Eu;
const GL_TEXTURE_ALPHA_SIZE             : GLenum = 0x805Fu;

const GL_DONT_CARE                      : GLenum = 0x1100u;
const GL_FASTEST                        : GLenum = 0x1101u;
const GL_NICEST                         : GLenum = 0x1102u;

const GL_BYTE                           : GLenum = 0x1400u;
const GL_UNSIGNED_BYTE                  : GLenum = 0x1401u;
const GL_SHORT                          : GLenum = 0x1402u;
const GL_UNSIGNED_SHORT                 : GLenum = 0x1403u;
const GL_INT                            : GLenum = 0x1404u;
const GL_UNSIGNED_INT                   : GLenum = 0x1405u;
const GL_FLOAT                          : GLenum = 0x1406u;
const GL_DOUBLE                         : GLenum = 0x140Au;

const GL_CLEAR                          : GLenum = 0x1500u;
const GL_AND                            : GLenum = 0x1501u;
const GL_AND_REVERSE                    : GLenum = 0x1502u;
const GL_COPY                           : GLenum = 0x1503u;
const GL_AND_INVERTED                   : GLenum = 0x1504u;
const GL_NOOP                           : GLenum = 0x1505u;
const GL_XOR                            : GLenum = 0x1506u;
const GL_OR                             : GLenum = 0x1507u;
const GL_NOR                            : GLenum = 0x1508u;
const GL_EQUIV                          : GLenum = 0x1509u;
const GL_INVERT                         : GLenum = 0x150Au;
const GL_OR_REVERSE                     : GLenum = 0x150Bu;
const GL_COPY_INVERTED                  : GLenum = 0x150Cu;
const GL_OR_INVERTED                    : GLenum = 0x150Du;
const GL_NAND                           : GLenum = 0x150Eu;
const GL_SET                            : GLenum = 0x150Fu;

const GL_TEXTURE                        : GLenum = 0x1702u;

const GL_COLOR                          : GLenum = 0x1800u;
const GL_DEPTH                          : GLenum = 0x1801u;
const GL_STENCIL                        : GLenum = 0x1802u;

const GL_STENCIL_INDEX                  : GLenum = 0x1901u;
const GL_DEPTH_COMPONENT                : GLenum = 0x1902u;
const GL_RED                            : GLenum = 0x1903u;
const GL_GREEN                          : GLenum = 0x1904u;
const GL_BLUE                           : GLenum = 0x1905u;
const GL_ALPHA                          : GLenum = 0x1906u;
const GL_RGB                            : GLenum = 0x1907u;
const GL_RGBA                           : GLenum = 0x1908u;

const GL_POINT                          : GLenum = 0x1B00u;
const GL_LINE                           : GLenum = 0x1B01u;
const GL_FILL                           : GLenum = 0x1B02u;

const GL_KEEP                           : GLenum = 0x1E00u;
const GL_REPLACE                        : GLenum = 0x1E01u;
const GL_INCR                           : GLenum = 0x1E02u;
const GL_DECR                           : GLenum = 0x1E03u;

const GL_VENDOR                         : GLenum = 0x1F00u;
const GL_RENDERER                       : GLenum = 0x1F01u;
const GL_VERSION                        : GLenum = 0x1F02u;
const GL_EXTENSIONS                     : GLenum = 0x1F03u;

const GL_NEAREST                        : GLenum = 0x2600u;
const GL_LINEAR                         : GLenum = 0x2601u;

const GL_NEAREST_MIPMAP_NEAREST         : GLenum = 0x2700u;
const GL_LINEAR_MIPMAP_NEAREST          : GLenum = 0x2701u;
const GL_NEAREST_MIPMAP_LINEAR          : GLenum = 0x2702u;
const GL_LINEAR_MIPMAP_LINEAR           : GLenum = 0x2703u;

const GL_TEXTURE_MAG_FILTER             : GLenum = 0x2800u;
const GL_TEXTURE_MIN_FILTER             : GLenum = 0x2801u;
const GL_TEXTURE_WRAP_S                 : GLenum = 0x2802u;
const GL_TEXTURE_WRAP_T                 : GLenum = 0x2803u;

const GL_PROXY_TEXTURE_1D               : GLenum = 0x8063u;
const GL_PROXY_TEXTURE_2D               : GLenum = 0x8064u;

const GL_REPEAT                         : GLenum = 0x2901u;

const GL_R3_G3_B2                       : GLenum = 0x2A10u;
const GL_RGB4                           : GLenum = 0x804Fu;
const GL_RGB5                           : GLenum = 0x8050u;
const GL_RGB8                           : GLenum = 0x8051u;
const GL_RGB10                          : GLenum = 0x8052u;
const GL_RGB12                          : GLenum = 0x8053u;
const GL_RGB16                          : GLenum = 0x8054u;
const GL_RGBA2                          : GLenum = 0x8055u;
const GL_RGBA4                          : GLenum = 0x8056u;
const GL_RGB5_A1                        : GLenum = 0x8057u;
const GL_RGBA8                          : GLenum = 0x8058u;
const GL_RGB10_A2                       : GLenum = 0x8059u;
const GL_RGBA12                         : GLenum = 0x805Au;
const GL_RGBA16                         : GLenum = 0x805Bu;

const GL_UNSIGNED_BYTE_3_3_2            : GLenum = 0x8032u;
const GL_UNSIGNED_SHORT_4_4_4_4         : GLenum = 0x8033u;
const GL_UNSIGNED_SHORT_5_5_5_1         : GLenum = 0x8034u;
const GL_UNSIGNED_INT_8_8_8_8           : GLenum = 0x8035u;
const GL_UNSIGNED_INT_10_10_10_2        : GLenum = 0x8036u;
const GL_TEXTURE_BINDING_3D             : GLenum = 0x806Au;
const GL_PACK_SKIP_IMAGES               : GLenum = 0x806Bu;
const GL_PACK_IMAGE_HEIGHT              : GLenum = 0x806Cu;
const GL_UNPACK_SKIP_IMAGES             : GLenum = 0x806Du;
const GL_UNPACK_IMAGE_HEIGHT            : GLenum = 0x806Eu;
const GL_TEXTURE_3D                     : GLenum = 0x806Fu;
const GL_PROXY_TEXTURE_3D               : GLenum = 0x8070u;
const GL_TEXTURE_DEPTH                  : GLenum = 0x8071u;
const GL_TEXTURE_WRAP_R                 : GLenum = 0x8072u;
const GL_MAX_3D_TEXTURE_SIZE            : GLenum = 0x8073u;
const GL_UNSIGNED_BYTE_2_3_3_REV        : GLenum = 0x8362u;
const GL_UNSIGNED_SHORT_5_6_5           : GLenum = 0x8363u;
const GL_UNSIGNED_SHORT_5_6_5_REV       : GLenum = 0x8364u;
const GL_UNSIGNED_SHORT_4_4_4_4_REV     : GLenum = 0x8365u;
const GL_UNSIGNED_SHORT_1_5_5_5_REV     : GLenum = 0x8366u;
const GL_UNSIGNED_INT_8_8_8_8_REV       : GLenum = 0x8367u;
const GL_UNSIGNED_INT_2_10_10_10_REV    : GLenum = 0x8368u;
const GL_BGR                            : GLenum = 0x80E0u;
const GL_BGRA                           : GLenum = 0x80E1u;
const GL_MAX_ELEMENTS_VERTICES          : GLenum = 0x80E8u;
const GL_MAX_ELEMENTS_INDICES           : GLenum = 0x80E9u;
const GL_CLAMP_TO_EDGE                  : GLenum = 0x812Fu;
const GL_TEXTURE_MIN_LOD                : GLenum = 0x813Au;
const GL_TEXTURE_MAX_LOD                : GLenum = 0x813Bu;
const GL_TEXTURE_BASE_LEVEL             : GLenum = 0x813Cu;
const GL_TEXTURE_MAX_LEVEL              : GLenum = 0x813Du;
const GL_SMOOTH_POINT_SIZE_RANGE        : GLenum = 0x0B12u;
const GL_SMOOTH_POINT_SIZE_GRANULARITY  : GLenum = 0x0B13u;
const GL_SMOOTH_LINE_WIDTH_RANGE        : GLenum = 0x0B22u;
const GL_SMOOTH_LINE_WIDTH_GRANULARITY  : GLenum = 0x0B23u;
const GL_ALIASED_LINE_WIDTH_RANGE       : GLenum = 0x846Eu;

const GL_CONSTANT_COLOR                 : GLenum = 0x8001u;
const GL_ONE_MINUS_CONSTANT_COLOR       : GLenum = 0x8002u;
const GL_CONSTANT_ALPHA                 : GLenum = 0x8003u;
const GL_ONE_MINUS_CONSTANT_ALPHA       : GLenum = 0x8004u;
const GL_BLEND_COLOR                    : GLenum = 0x8005u;
const GL_FUNC_ADD                       : GLenum = 0x8006u;
const GL_MIN                            : GLenum = 0x8007u;
const GL_MAX                            : GLenum = 0x8008u;
const GL_BLEND_EQUATION                 : GLenum = 0x8009u;
const GL_FUNC_SUBTRACT                  : GLenum = 0x800Au;
const GL_FUNC_REVERSE_SUBTRACT          : GLenum = 0x800Bu;

const GL_TEXTURE0                       : GLenum = 0x84C0u;
const GL_TEXTURE1                       : GLenum = 0x84C1u;
const GL_TEXTURE2                       : GLenum = 0x84C2u;
const GL_TEXTURE3                       : GLenum = 0x84C3u;
const GL_TEXTURE4                       : GLenum = 0x84C4u;
const GL_TEXTURE5                       : GLenum = 0x84C5u;
const GL_TEXTURE6                       : GLenum = 0x84C6u;
const GL_TEXTURE7                       : GLenum = 0x84C7u;
const GL_TEXTURE8                       : GLenum = 0x84C8u;
const GL_TEXTURE9                       : GLenum = 0x84C9u;
const GL_TEXTURE10                      : GLenum = 0x84CAu;
const GL_TEXTURE11                      : GLenum = 0x84CBu;
const GL_TEXTURE12                      : GLenum = 0x84CCu;
const GL_TEXTURE13                      : GLenum = 0x84CDu;
const GL_TEXTURE14                      : GLenum = 0x84CEu;
const GL_TEXTURE15                      : GLenum = 0x84CFu;
const GL_TEXTURE16                      : GLenum = 0x84D0u;
const GL_TEXTURE17                      : GLenum = 0x84D1u;
const GL_TEXTURE18                      : GLenum = 0x84D2u;
const GL_TEXTURE19                      : GLenum = 0x84D3u;
const GL_TEXTURE20                      : GLenum = 0x84D4u;
const GL_TEXTURE21                      : GLenum = 0x84D5u;
const GL_TEXTURE22                      : GLenum = 0x84D6u;
const GL_TEXTURE23                      : GLenum = 0x84D7u;
const GL_TEXTURE24                      : GLenum = 0x84D8u;
const GL_TEXTURE25                      : GLenum = 0x84D9u;
const GL_TEXTURE26                      : GLenum = 0x84DAu;
const GL_TEXTURE27                      : GLenum = 0x84DBu;
const GL_TEXTURE28                      : GLenum = 0x84DCu;
const GL_TEXTURE29                      : GLenum = 0x84DDu;
const GL_TEXTURE30                      : GLenum = 0x84DEu;
const GL_TEXTURE31                      : GLenum = 0x84DFu;
const GL_ACTIVE_TEXTURE                 : GLenum = 0x84E0u;
const GL_MULTISAMPLE                    : GLenum = 0x809Du;
const GL_SAMPLE_ALPHA_TO_COVERAGE       : GLenum = 0x809Eu;
const GL_SAMPLE_ALPHA_TO_ONE            : GLenum = 0x809Fu;
const GL_SAMPLE_COVERAGE                : GLenum = 0x80A0u;
const GL_SAMPLE_BUFFERS                 : GLenum = 0x80A8u;
const GL_SAMPLES                        : GLenum = 0x80A9u;
const GL_SAMPLE_COVERAGE_VALUE          : GLenum = 0x80AAu;
const GL_SAMPLE_COVERAGE_INVERT         : GLenum = 0x80ABu;
const GL_TEXTURE_CUBE_MAP               : GLenum = 0x8513u;
const GL_TEXTURE_BINDING_CUBE_MAP       : GLenum = 0x8514u;
const GL_TEXTURE_CUBE_MAP_POSITIVE_X    : GLenum = 0x8515u;
const GL_TEXTURE_CUBE_MAP_NEGATIVE_X    : GLenum = 0x8516u;
const GL_TEXTURE_CUBE_MAP_POSITIVE_Y    : GLenum = 0x8517u;
const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y    : GLenum = 0x8518u;
const GL_TEXTURE_CUBE_MAP_POSITIVE_Z    : GLenum = 0x8519u;
const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z    : GLenum = 0x851Au;
const GL_PROXY_TEXTURE_CUBE_MAP         : GLenum = 0x851Bu;
const GL_MAX_CUBE_MAP_TEXTURE_SIZE      : GLenum = 0x851Cu;
const GL_COMPRESSED_RGB                 : GLenum = 0x84EDu;
const GL_COMPRESSED_RGBA                : GLenum = 0x84EEu;
const GL_TEXTURE_COMPRESSION_HINT       : GLenum = 0x84EFu;
const GL_TEXTURE_COMPRESSED_IMAGE_SIZE  : GLenum = 0x86A0u;
const GL_TEXTURE_COMPRESSED             : GLenum = 0x86A1u;
const GL_NUM_COMPRESSED_TEXTURE_FORMATS : GLenum = 0x86A2u;
const GL_COMPRESSED_TEXTURE_FORMATS     : GLenum = 0x86A3u;
const GL_CLAMP_TO_BORDER                : GLenum = 0x812Du;

const GL_BLEND_DST_RGB                  : GLenum = 0x80C8u;
const GL_BLEND_SRC_RGB                  : GLenum = 0x80C9u;
const GL_BLEND_DST_ALPHA                : GLenum = 0x80CAu;
const GL_BLEND_SRC_ALPHA                : GLenum = 0x80CBu;
const GL_POINT_FADE_THRESHOLD_SIZE      : GLenum = 0x8128u;
const GL_DEPTH_COMPONENT16              : GLenum = 0x81A5u;
const GL_DEPTH_COMPONENT24              : GLenum = 0x81A6u;
const GL_DEPTH_COMPONENT32              : GLenum = 0x81A7u;
const GL_MIRRORED_REPEAT                : GLenum = 0x8370u;
const GL_MAX_TEXTURE_LOD_BIAS           : GLenum = 0x84FDu;
const GL_TEXTURE_LOD_BIAS               : GLenum = 0x8501u;
const GL_INCR_WRAP                      : GLenum = 0x8507u;
const GL_DECR_WRAP                      : GLenum = 0x8508u;
const GL_TEXTURE_DEPTH_SIZE             : GLenum = 0x884Au;
const GL_TEXTURE_COMPARE_MODE           : GLenum = 0x884Cu;
const GL_TEXTURE_COMPARE_FUNC           : GLenum = 0x884Du;

const GL_BUFFER_SIZE                    : GLenum = 0x8764u;
const GL_BUFFER_USAGE                   : GLenum = 0x8765u;
const GL_QUERY_COUNTER_BITS             : GLenum = 0x8864u;
const GL_CURRENT_QUERY                  : GLenum = 0x8865u;
const GL_QUERY_RESULT                   : GLenum = 0x8866u;
const GL_QUERY_RESULT_AVAILABLE         : GLenum = 0x8867u;
const GL_ARRAY_BUFFER                   : GLenum = 0x8892u;
const GL_ELEMENT_ARRAY_BUFFER           : GLenum = 0x8893u;
const GL_ARRAY_BUFFER_BINDING           : GLenum = 0x8894u;
const GL_ELEMENT_ARRAY_BUFFER_BINDING   : GLenum = 0x8895u;
const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING : GLenum = 0x889Fu;
const GL_READ_ONLY                      : GLenum = 0x88B8u;
const GL_WRITE_ONLY                     : GLenum = 0x88B9u;
const GL_READ_WRITE                     : GLenum = 0x88BAu;
const GL_BUFFER_ACCESS                  : GLenum = 0x88BBu;
const GL_BUFFER_MAPPED                  : GLenum = 0x88BCu;
const GL_BUFFER_MAP_POINTER             : GLenum = 0x88BDu;
const GL_STREAM_DRAW                    : GLenum = 0x88E0u;
const GL_STREAM_READ                    : GLenum = 0x88E1u;
const GL_STREAM_COPY                    : GLenum = 0x88E2u;
const GL_STATIC_DRAW                    : GLenum = 0x88E4u;
const GL_STATIC_READ                    : GLenum = 0x88E5u;
const GL_STATIC_COPY                    : GLenum = 0x88E6u;
const GL_DYNAMIC_DRAW                   : GLenum = 0x88E8u;
const GL_DYNAMIC_READ                   : GLenum = 0x88E9u;
const GL_DYNAMIC_COPY                   : GLenum = 0x88EAu;
const GL_SAMPLES_PASSED                 : GLenum = 0x8914u;

const GL_BLEND_EQUATION_RGB             : GLenum = 0x8009u;
const GL_VERTEX_ATTRIB_ARRAY_ENABLED    : GLenum = 0x8622u;
const GL_VERTEX_ATTRIB_ARRAY_SIZE       : GLenum = 0x8623u;
const GL_VERTEX_ATTRIB_ARRAY_STRIDE     : GLenum = 0x8624u;
const GL_VERTEX_ATTRIB_ARRAY_TYPE       : GLenum = 0x8625u;
const GL_CURRENT_VERTEX_ATTRIB          : GLenum = 0x8626u;
const GL_VERTEX_PROGRAM_POINT_SIZE      : GLenum = 0x8642u;
const GL_VERTEX_ATTRIB_ARRAY_POINTER    : GLenum = 0x8645u;
const GL_STENCIL_BACK_FUNC              : GLenum = 0x8800u;
const GL_STENCIL_BACK_FAIL              : GLenum = 0x8801u;
const GL_STENCIL_BACK_PASS_DEPTH_FAIL   : GLenum = 0x8802u;
const GL_STENCIL_BACK_PASS_DEPTH_PASS   : GLenum = 0x8803u;
const GL_MAX_DRAW_BUFFERS               : GLenum = 0x8824u;
const GL_DRAW_BUFFER0                   : GLenum = 0x8825u;
const GL_DRAW_BUFFER1                   : GLenum = 0x8826u;
const GL_DRAW_BUFFER2                   : GLenum = 0x8827u;
const GL_DRAW_BUFFER3                   : GLenum = 0x8828u;
const GL_DRAW_BUFFER4                   : GLenum = 0x8829u;
const GL_DRAW_BUFFER5                   : GLenum = 0x882Au;
const GL_DRAW_BUFFER6                   : GLenum = 0x882Bu;
const GL_DRAW_BUFFER7                   : GLenum = 0x882Cu;
const GL_DRAW_BUFFER8                   : GLenum = 0x882Du;
const GL_DRAW_BUFFER9                   : GLenum = 0x882Eu;
const GL_DRAW_BUFFER10                  : GLenum = 0x882Fu;
const GL_DRAW_BUFFER11                  : GLenum = 0x8830u;
const GL_DRAW_BUFFER12                  : GLenum = 0x8831u;
const GL_DRAW_BUFFER13                  : GLenum = 0x8832u;
const GL_DRAW_BUFFER14                  : GLenum = 0x8833u;
const GL_DRAW_BUFFER15                  : GLenum = 0x8834u;
const GL_BLEND_EQUATION_ALPHA           : GLenum = 0x883Du;
const GL_MAX_VERTEX_ATTRIBS             : GLenum = 0x8869u;
const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED : GLenum = 0x886Au;
const GL_MAX_TEXTURE_IMAGE_UNITS        : GLenum = 0x8872u;
const GL_FRAGMENT_SHADER                : GLenum = 0x8B30u;
const GL_VERTEX_SHADER                  : GLenum = 0x8B31u;
const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS : GLenum = 0x8B49u;
const GL_MAX_VERTEX_UNIFORM_COMPONENTS  : GLenum = 0x8B4Au;
const GL_MAX_VARYING_FLOATS             : GLenum = 0x8B4Bu;
const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS : GLenum = 0x8B4Cu;
const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS : GLenum = 0x8B4Du;
const GL_SHADER_TYPE                    : GLenum = 0x8B4Fu;
const GL_FLOAT_VEC2                     : GLenum = 0x8B50u;
const GL_FLOAT_VEC3                     : GLenum = 0x8B51u;
const GL_FLOAT_VEC4                     : GLenum = 0x8B52u;
const GL_INT_VEC2                       : GLenum = 0x8B53u;
const GL_INT_VEC3                       : GLenum = 0x8B54u;
const GL_INT_VEC4                       : GLenum = 0x8B55u;
const GL_BOOL                           : GLenum = 0x8B56u;
const GL_BOOL_VEC2                      : GLenum = 0x8B57u;
const GL_BOOL_VEC3                      : GLenum = 0x8B58u;
const GL_BOOL_VEC4                      : GLenum = 0x8B59u;
const GL_FLOAT_MAT2                     : GLenum = 0x8B5Au;
const GL_FLOAT_MAT3                     : GLenum = 0x8B5Bu;
const GL_FLOAT_MAT4                     : GLenum = 0x8B5Cu;
const GL_SAMPLER_1D                     : GLenum = 0x8B5Du;
const GL_SAMPLER_2D                     : GLenum = 0x8B5Eu;
const GL_SAMPLER_3D                     : GLenum = 0x8B5Fu;
const GL_SAMPLER_CUBE                   : GLenum = 0x8B60u;
const GL_SAMPLER_1D_SHADOW              : GLenum = 0x8B61u;
const GL_SAMPLER_2D_SHADOW              : GLenum = 0x8B62u;
const GL_DELETE_STATUS                  : GLenum = 0x8B80u;
const GL_COMPILE_STATUS                 : GLenum = 0x8B81u;
const GL_LINK_STATUS                    : GLenum = 0x8B82u;
const GL_VALIDATE_STATUS                : GLenum = 0x8B83u;
const GL_INFO_LOG_LENGTH                : GLenum = 0x8B84u;
const GL_ATTACHED_SHADERS               : GLenum = 0x8B85u;
const GL_ACTIVE_UNIFORMS                : GLenum = 0x8B86u;
const GL_ACTIVE_UNIFORM_MAX_LENGTH      : GLenum = 0x8B87u;
const GL_SHADER_SOURCE_LENGTH           : GLenum = 0x8B88u;
const GL_ACTIVE_ATTRIBUTES              : GLenum = 0x8B89u;
const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH    : GLenum = 0x8B8Au;
const GL_FRAGMENT_SHADER_DERIVATIVE_HINT : GLenum = 0x8B8Bu;
const GL_SHADING_LANGUAGE_VERSION       : GLenum = 0x8B8Cu;
const GL_CURRENT_PROGRAM                : GLenum = 0x8B8Du;
const GL_POINT_SPRITE_COORD_ORIGIN      : GLenum = 0x8CA0u;
const GL_LOWER_LEFT                     : GLenum = 0x8CA1u;
const GL_UPPER_LEFT                     : GLenum = 0x8CA2u;
const GL_STENCIL_BACK_REF               : GLenum = 0x8CA3u;
const GL_STENCIL_BACK_VALUE_MASK        : GLenum = 0x8CA4u;
const GL_STENCIL_BACK_WRITEMASK         : GLenum = 0x8CA5u;

const GL_PIXEL_PACK_BUFFER              : GLenum = 0x88EBu;
const GL_PIXEL_UNPACK_BUFFER            : GLenum = 0x88ECu;
const GL_PIXEL_PACK_BUFFER_BINDING      : GLenum = 0x88EDu;
const GL_PIXEL_UNPACK_BUFFER_BINDING    : GLenum = 0x88EFu;
const GL_FLOAT_MAT2x3                   : GLenum = 0x8B65u;
const GL_FLOAT_MAT2x4                   : GLenum = 0x8B66u;
const GL_FLOAT_MAT3x2                   : GLenum = 0x8B67u;
const GL_FLOAT_MAT3x4                   : GLenum = 0x8B68u;
const GL_FLOAT_MAT4x2                   : GLenum = 0x8B69u;
const GL_FLOAT_MAT4x3                   : GLenum = 0x8B6Au;
const GL_SRGB                           : GLenum = 0x8C40u;
const GL_SRGB8                          : GLenum = 0x8C41u;
const GL_SRGB_ALPHA                     : GLenum = 0x8C42u;
const GL_SRGB8_ALPHA8                   : GLenum = 0x8C43u;
const GL_COMPRESSED_SRGB                : GLenum = 0x8C48u;
const GL_COMPRESSED_SRGB_ALPHA          : GLenum = 0x8C49u;

const GL_COMPARE_REF_TO_TEXTURE         : GLenum = 0x884Eu;
const GL_CLIP_DISTANCE0                 : GLenum = 0x3000u;
const GL_CLIP_DISTANCE1                 : GLenum = 0x3001u;
const GL_CLIP_DISTANCE2                 : GLenum = 0x3002u;
const GL_CLIP_DISTANCE3                 : GLenum = 0x3003u;
const GL_CLIP_DISTANCE4                 : GLenum = 0x3004u;
const GL_CLIP_DISTANCE5                 : GLenum = 0x3005u;
const GL_CLIP_DISTANCE6                 : GLenum = 0x3006u;
const GL_CLIP_DISTANCE7                 : GLenum = 0x3007u;
const GL_MAX_CLIP_DISTANCES             : GLenum = 0x0D32u;
const GL_MAJOR_VERSION                  : GLenum = 0x821Bu;
const GL_MINOR_VERSION                  : GLenum = 0x821Cu;
const GL_NUM_EXTENSIONS                 : GLenum = 0x821Du;
const GL_CONTEXT_FLAGS                  : GLenum = 0x821Eu;
const GL_COMPRESSED_RED                 : GLenum = 0x8225u;
const GL_COMPRESSED_RG                  : GLenum = 0x8226u;
const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT : GLenum = 0x0001u;
const GL_RGBA32F                        : GLenum = 0x8814u;
const GL_RGB32F                         : GLenum = 0x8815u;
const GL_RGBA16F                        : GLenum = 0x881Au;
const GL_RGB16F                         : GLenum = 0x881Bu;
const GL_VERTEX_ATTRIB_ARRAY_INTEGER    : GLenum = 0x88FDu;
const GL_MAX_ARRAY_TEXTURE_LAYERS       : GLenum = 0x88FFu;
const GL_MIN_PROGRAM_TEXEL_OFFSET       : GLenum = 0x8904u;
const GL_MAX_PROGRAM_TEXEL_OFFSET       : GLenum = 0x8905u;
const GL_CLAMP_READ_COLOR               : GLenum = 0x891Cu;
const GL_FIXED_ONLY                     : GLenum = 0x891Du;
const GL_MAX_VARYING_COMPONENTS         : GLenum = 0x8B4Bu;
const GL_TEXTURE_1D_ARRAY               : GLenum = 0x8C18u;
const GL_PROXY_TEXTURE_1D_ARRAY         : GLenum = 0x8C19u;
const GL_TEXTURE_2D_ARRAY               : GLenum = 0x8C1Au;
const GL_PROXY_TEXTURE_2D_ARRAY         : GLenum = 0x8C1Bu;
const GL_TEXTURE_BINDING_1D_ARRAY       : GLenum = 0x8C1Cu;
const GL_TEXTURE_BINDING_2D_ARRAY       : GLenum = 0x8C1Du;
const GL_R11F_G11F_B10F                 : GLenum = 0x8C3Au;
const GL_UNSIGNED_INT_10F_11F_11F_REV   : GLenum = 0x8C3Bu;
const GL_RGB9_E5                        : GLenum = 0x8C3Du;
const GL_UNSIGNED_INT_5_9_9_9_REV       : GLenum = 0x8C3Eu;
const GL_TEXTURE_SHARED_SIZE            : GLenum = 0x8C3Fu;
const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH : GLenum = 0x8C76u;
const GL_TRANSFORM_FEEDBACK_BUFFER_MODE : GLenum = 0x8C7Fu;
const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS : GLenum = 0x8C80u;
const GL_TRANSFORM_FEEDBACK_VARYINGS    : GLenum = 0x8C83u;
const GL_TRANSFORM_FEEDBACK_BUFFER_START : GLenum = 0x8C84u;
const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE : GLenum = 0x8C85u;
const GL_PRIMITIVES_GENERATED           : GLenum = 0x8C87u;
const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN : GLenum = 0x8C88u;
const GL_RASTERIZER_DISCARD             : GLenum = 0x8C89u;
const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS : GLenum = 0x8C8Au;
const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS : GLenum = 0x8C8Bu;
const GL_INTERLEAVED_ATTRIBS            : GLenum = 0x8C8Cu;
const GL_SEPARATE_ATTRIBS               : GLenum = 0x8C8Du;
const GL_TRANSFORM_FEEDBACK_BUFFER      : GLenum = 0x8C8Eu;
const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING : GLenum = 0x8C8Fu;
const GL_RGBA32UI                       : GLenum = 0x8D70u;
const GL_RGB32UI                        : GLenum = 0x8D71u;
const GL_RGBA16UI                       : GLenum = 0x8D76u;
const GL_RGB16UI                        : GLenum = 0x8D77u;
const GL_RGBA8UI                        : GLenum = 0x8D7Cu;
const GL_RGB8UI                         : GLenum = 0x8D7Du;
const GL_RGBA32I                        : GLenum = 0x8D82u;
const GL_RGB32I                         : GLenum = 0x8D83u;
const GL_RGBA16I                        : GLenum = 0x8D88u;
const GL_RGB16I                         : GLenum = 0x8D89u;
const GL_RGBA8I                         : GLenum = 0x8D8Eu;
const GL_RGB8I                          : GLenum = 0x8D8Fu;
const GL_RED_INTEGER                    : GLenum = 0x8D94u;
const GL_GREEN_INTEGER                  : GLenum = 0x8D95u;
const GL_BLUE_INTEGER                   : GLenum = 0x8D96u;
const GL_RGB_INTEGER                    : GLenum = 0x8D98u;
const GL_RGBA_INTEGER                   : GLenum = 0x8D99u;
const GL_BGR_INTEGER                    : GLenum = 0x8D9Au;
const GL_BGRA_INTEGER                   : GLenum = 0x8D9Bu;
const GL_SAMPLER_1D_ARRAY               : GLenum = 0x8DC0u;
const GL_SAMPLER_2D_ARRAY               : GLenum = 0x8DC1u;
const GL_SAMPLER_1D_ARRAY_SHADOW        : GLenum = 0x8DC3u;
const GL_SAMPLER_2D_ARRAY_SHADOW        : GLenum = 0x8DC4u;
const GL_SAMPLER_CUBE_SHADOW            : GLenum = 0x8DC5u;
const GL_UNSIGNED_INT_VEC2              : GLenum = 0x8DC6u;
const GL_UNSIGNED_INT_VEC3              : GLenum = 0x8DC7u;
const GL_UNSIGNED_INT_VEC4              : GLenum = 0x8DC8u;
const GL_INT_SAMPLER_1D                 : GLenum = 0x8DC9u;
const GL_INT_SAMPLER_2D                 : GLenum = 0x8DCAu;
const GL_INT_SAMPLER_3D                 : GLenum = 0x8DCBu;
const GL_INT_SAMPLER_CUBE               : GLenum = 0x8DCCu;
const GL_INT_SAMPLER_1D_ARRAY           : GLenum = 0x8DCEu;
const GL_INT_SAMPLER_2D_ARRAY           : GLenum = 0x8DCFu;
const GL_UNSIGNED_INT_SAMPLER_1D        : GLenum = 0x8DD1u;
const GL_UNSIGNED_INT_SAMPLER_2D        : GLenum = 0x8DD2u;
const GL_UNSIGNED_INT_SAMPLER_3D        : GLenum = 0x8DD3u;
const GL_UNSIGNED_INT_SAMPLER_CUBE      : GLenum = 0x8DD4u;
const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY  : GLenum = 0x8DD6u;
const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY  : GLenum = 0x8DD7u;
const GL_QUERY_WAIT                     : GLenum = 0x8E13u;
const GL_QUERY_NO_WAIT                  : GLenum = 0x8E14u;
const GL_QUERY_BY_REGION_WAIT           : GLenum = 0x8E15u;
const GL_QUERY_BY_REGION_NO_WAIT        : GLenum = 0x8E16u;
const GL_BUFFER_ACCESS_FLAGS            : GLenum = 0x911Fu;
const GL_BUFFER_MAP_LENGTH              : GLenum = 0x9120u;
const GL_BUFFER_MAP_OFFSET              : GLenum = 0x9121u;

const GL_SAMPLER_2D_RECT                : GLenum = 0x8B63u;
const GL_SAMPLER_2D_RECT_SHADOW         : GLenum = 0x8B64u;
const GL_SAMPLER_BUFFER                 : GLenum = 0x8DC2u;
const GL_INT_SAMPLER_2D_RECT            : GLenum = 0x8DCDu;
const GL_INT_SAMPLER_BUFFER             : GLenum = 0x8DD0u;
const GL_UNSIGNED_INT_SAMPLER_2D_RECT   : GLenum = 0x8DD5u;
const GL_UNSIGNED_INT_SAMPLER_BUFFER    : GLenum = 0x8DD8u;
const GL_TEXTURE_BUFFER                 : GLenum = 0x8C2Au;
const GL_MAX_TEXTURE_BUFFER_SIZE        : GLenum = 0x8C2Bu;
const GL_TEXTURE_BINDING_BUFFER         : GLenum = 0x8C2Cu;
const GL_TEXTURE_BUFFER_DATA_STORE_BINDING : GLenum = 0x8C2Du;
const GL_TEXTURE_BUFFER_FORMAT          : GLenum = 0x8C2Eu;
const GL_TEXTURE_RECTANGLE              : GLenum = 0x84F5u;
const GL_TEXTURE_BINDING_RECTANGLE      : GLenum = 0x84F6u;
const GL_PROXY_TEXTURE_RECTANGLE        : GLenum = 0x84F7u;
const GL_MAX_RECTANGLE_TEXTURE_SIZE     : GLenum = 0x84F8u;
const GL_RED_SNORM                      : GLenum = 0x8F90u;
const GL_RG_SNORM                       : GLenum = 0x8F91u;
const GL_RGB_SNORM                      : GLenum = 0x8F92u;
const GL_RGBA_SNORM                     : GLenum = 0x8F93u;
const GL_R8_SNORM                       : GLenum = 0x8F94u;
const GL_RG8_SNORM                      : GLenum = 0x8F95u;
const GL_RGB8_SNORM                     : GLenum = 0x8F96u;
const GL_RGBA8_SNORM                    : GLenum = 0x8F97u;
const GL_R16_SNORM                      : GLenum = 0x8F98u;
const GL_RG16_SNORM                     : GLenum = 0x8F99u;
const GL_RGB16_SNORM                    : GLenum = 0x8F9Au;
const GL_RGBA16_SNORM                   : GLenum = 0x8F9Bu;
const GL_SIGNED_NORMALIZED              : GLenum = 0x8F9Cu;
const GL_PRIMITIVE_RESTART              : GLenum = 0x8F9Du;
const GL_PRIMITIVE_RESTART_INDEX        : GLenum = 0x8F9Eu;

const GL_CONTEXT_CORE_PROFILE_BIT       : GLenum = 0x00000001u;
const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT : GLenum = 0x00000002u;
const GL_LINES_ADJACENCY                : GLenum = 0x000Au;
const GL_LINE_STRIP_ADJACENCY           : GLenum = 0x000Bu;
const GL_TRIANGLES_ADJACENCY            : GLenum = 0x000Cu;
const GL_TRIANGLE_STRIP_ADJACENCY       : GLenum = 0x000Du;
const GL_PROGRAM_POINT_SIZE             : GLenum = 0x8642u;
const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS : GLenum = 0x8C29u;
const GL_FRAMEBUFFER_ATTACHMENT_LAYERED : GLenum = 0x8DA7u;
const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS : GLenum = 0x8DA8u;
const GL_GEOMETRY_SHADER                : GLenum = 0x8DD9u;
const GL_GEOMETRY_VERTICES_OUT          : GLenum = 0x8916u;
const GL_GEOMETRY_INPUT_TYPE            : GLenum = 0x8917u;
const GL_GEOMETRY_OUTPUT_TYPE           : GLenum = 0x8918u;
const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS : GLenum = 0x8DDFu;
const GL_MAX_GEOMETRY_OUTPUT_VERTICES   : GLenum = 0x8DE0u;
const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS : GLenum = 0x8DE1u;
const GL_MAX_VERTEX_OUTPUT_COMPONENTS   : GLenum = 0x9122u;
const GL_MAX_GEOMETRY_INPUT_COMPONENTS  : GLenum = 0x9123u;
const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS : GLenum = 0x9124u;
const GL_MAX_FRAGMENT_INPUT_COMPONENTS  : GLenum = 0x9125u;
const GL_CONTEXT_PROFILE_MASK           : GLenum = 0x9126u;

const GL_VERTEX_ATTRIB_ARRAY_DIVISOR    : GLenum = 0x88FEu;

const GL_SAMPLE_SHADING                 : GLenum = 0x8C36u;
const GL_MIN_SAMPLE_SHADING_VALUE       : GLenum = 0x8C37u;
const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET : GLenum = 0x8E5Eu;
const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET : GLenum = 0x8E5Fu;
const GL_TEXTURE_CUBE_MAP_ARRAY         : GLenum = 0x9009u;
const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY : GLenum = 0x900Au;
const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY   : GLenum = 0x900Bu;
const GL_SAMPLER_CUBE_MAP_ARRAY         : GLenum = 0x900Cu;
const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW  : GLenum = 0x900Du;
const GL_INT_SAMPLER_CUBE_MAP_ARRAY     : GLenum = 0x900Eu;
const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY : GLenum = 0x900Fu;

const GL_DEPTH_COMPONENT32F             : GLenum = 0x8CACu;
const GL_DEPTH32F_STENCIL8              : GLenum = 0x8CADu;
const GL_FLOAT_32_UNSIGNED_INT_24_8_REV : GLenum = 0x8DADu;

const GL_INVALID_FRAMEBUFFER_OPERATION  : GLenum = 0x0506u;
const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING : GLenum = 0x8210u;
const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE : GLenum = 0x8211u;
const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE : GLenum = 0x8212u;
const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE : GLenum = 0x8213u;
const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE : GLenum = 0x8214u;
const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE : GLenum = 0x8215u;
const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE : GLenum = 0x8216u;
const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE : GLenum = 0x8217u;
const GL_FRAMEBUFFER_DEFAULT            : GLenum = 0x8218u;
const GL_FRAMEBUFFER_UNDEFINED          : GLenum = 0x8219u;
const GL_DEPTH_STENCIL_ATTACHMENT       : GLenum = 0x821Au;
const GL_MAX_RENDERBUFFER_SIZE          : GLenum = 0x84E8u;
const GL_DEPTH_STENCIL                  : GLenum = 0x84F9u;
const GL_UNSIGNED_INT_24_8              : GLenum = 0x84FAu;
const GL_DEPTH24_STENCIL8               : GLenum = 0x88F0u;
const GL_TEXTURE_STENCIL_SIZE           : GLenum = 0x88F1u;
const GL_TEXTURE_RED_TYPE               : GLenum = 0x8C10u;
const GL_TEXTURE_GREEN_TYPE             : GLenum = 0x8C11u;
const GL_TEXTURE_BLUE_TYPE              : GLenum = 0x8C12u;
const GL_TEXTURE_ALPHA_TYPE             : GLenum = 0x8C13u;
const GL_TEXTURE_DEPTH_TYPE             : GLenum = 0x8C16u;
const GL_UNSIGNED_NORMALIZED            : GLenum = 0x8C17u;
const GL_FRAMEBUFFER_BINDING            : GLenum = 0x8CA6u;
const GL_DRAW_FRAMEBUFFER_BINDING       : GLenum = 0x8CA6u;
const GL_RENDERBUFFER_BINDING           : GLenum = 0x8CA7u;
const GL_READ_FRAMEBUFFER               : GLenum = 0x8CA8u;
const GL_DRAW_FRAMEBUFFER               : GLenum = 0x8CA9u;
const GL_READ_FRAMEBUFFER_BINDING       : GLenum = 0x8CAAu;
const GL_RENDERBUFFER_SAMPLES           : GLenum = 0x8CABu;
const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE : GLenum = 0x8CD0u;
const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME : GLenum = 0x8CD1u;
const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL : GLenum = 0x8CD2u;
const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE : GLenum = 0x8CD3u;
const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER : GLenum = 0x8CD4u;
const GL_FRAMEBUFFER_COMPLETE           : GLenum = 0x8CD5u;
const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT : GLenum = 0x8CD6u;
const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT : GLenum = 0x8CD7u;
const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER : GLenum = 0x8CDBu;
const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER : GLenum = 0x8CDCu;
const GL_FRAMEBUFFER_UNSUPPORTED        : GLenum = 0x8CDDu;
const GL_MAX_COLOR_ATTACHMENTS          : GLenum = 0x8CDFu;
const GL_COLOR_ATTACHMENT0              : GLenum = 0x8CE0u;
const GL_COLOR_ATTACHMENT1              : GLenum = 0x8CE1u;
const GL_COLOR_ATTACHMENT2              : GLenum = 0x8CE2u;
const GL_COLOR_ATTACHMENT3              : GLenum = 0x8CE3u;
const GL_COLOR_ATTACHMENT4              : GLenum = 0x8CE4u;
const GL_COLOR_ATTACHMENT5              : GLenum = 0x8CE5u;
const GL_COLOR_ATTACHMENT6              : GLenum = 0x8CE6u;
const GL_COLOR_ATTACHMENT7              : GLenum = 0x8CE7u;
const GL_COLOR_ATTACHMENT8              : GLenum = 0x8CE8u;
const GL_COLOR_ATTACHMENT9              : GLenum = 0x8CE9u;
const GL_COLOR_ATTACHMENT10             : GLenum = 0x8CEAu;
const GL_COLOR_ATTACHMENT11             : GLenum = 0x8CEBu;
const GL_COLOR_ATTACHMENT12             : GLenum = 0x8CECu;
const GL_COLOR_ATTACHMENT13             : GLenum = 0x8CEDu;
const GL_COLOR_ATTACHMENT14             : GLenum = 0x8CEEu;
const GL_COLOR_ATTACHMENT15             : GLenum = 0x8CEFu;
const GL_DEPTH_ATTACHMENT               : GLenum = 0x8D00u;
const GL_STENCIL_ATTACHMENT             : GLenum = 0x8D20u;
const GL_FRAMEBUFFER                    : GLenum = 0x8D40u;
const GL_RENDERBUFFER                   : GLenum = 0x8D41u;
const GL_RENDERBUFFER_WIDTH             : GLenum = 0x8D42u;
const GL_RENDERBUFFER_HEIGHT            : GLenum = 0x8D43u;
const GL_RENDERBUFFER_INTERNAL_FORMAT   : GLenum = 0x8D44u;
const GL_STENCIL_INDEX1                 : GLenum = 0x8D46u;
const GL_STENCIL_INDEX4                 : GLenum = 0x8D47u;
const GL_STENCIL_INDEX8                 : GLenum = 0x8D48u;
const GL_STENCIL_INDEX16                : GLenum = 0x8D49u;
const GL_RENDERBUFFER_RED_SIZE          : GLenum = 0x8D50u;
const GL_RENDERBUFFER_GREEN_SIZE        : GLenum = 0x8D51u;
const GL_RENDERBUFFER_BLUE_SIZE         : GLenum = 0x8D52u;
const GL_RENDERBUFFER_ALPHA_SIZE        : GLenum = 0x8D53u;
const GL_RENDERBUFFER_DEPTH_SIZE        : GLenum = 0x8D54u;
const GL_RENDERBUFFER_STENCIL_SIZE      : GLenum = 0x8D55u;
const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE : GLenum = 0x8D56u;
const GL_MAX_SAMPLES                    : GLenum = 0x8D57u;

const GL_FRAMEBUFFER_SRGB               : GLenum = 0x8DB9u;

const GL_HALF_FLOAT                     : GLenum = 0x140Bu;

const GL_MAP_READ_BIT                   : GLenum = 0x0001u;
const GL_MAP_WRITE_BIT                  : GLenum = 0x0002u;
const GL_MAP_INVALIDATE_RANGE_BIT       : GLenum = 0x0004u;
const GL_MAP_INVALIDATE_BUFFER_BIT      : GLenum = 0x0008u;
const GL_MAP_FLUSH_EXPLICIT_BIT         : GLenum = 0x0010u;
const GL_MAP_UNSYNCHRONIZED_BIT         : GLenum = 0x0020u;

const GL_COMPRESSED_RED_RGTC1           : GLenum = 0x8DBBu;
const GL_COMPRESSED_SIGNED_RED_RGTC1    : GLenum = 0x8DBCu;
const GL_COMPRESSED_RG_RGTC2            : GLenum = 0x8DBDu;
const GL_COMPRESSED_SIGNED_RG_RGTC2     : GLenum = 0x8DBEu;

const GL_RG                             : GLenum = 0x8227u;
const GL_RG_INTEGER                     : GLenum = 0x8228u;
const GL_R8                             : GLenum = 0x8229u;
const GL_R16                            : GLenum = 0x822Au;
const GL_RG8                            : GLenum = 0x822Bu;
const GL_RG16                           : GLenum = 0x822Cu;
const GL_R16F                           : GLenum = 0x822Du;
const GL_R32F                           : GLenum = 0x822Eu;
const GL_RG16F                          : GLenum = 0x822Fu;
const GL_RG32F                          : GLenum = 0x8230u;
const GL_R8I                            : GLenum = 0x8231u;
const GL_R8UI                           : GLenum = 0x8232u;
const GL_R16I                           : GLenum = 0x8233u;
const GL_R16UI                          : GLenum = 0x8234u;
const GL_R32I                           : GLenum = 0x8235u;
const GL_R32UI                          : GLenum = 0x8236u;
const GL_RG8I                           : GLenum = 0x8237u;
const GL_RG8UI                          : GLenum = 0x8238u;
const GL_RG16I                          : GLenum = 0x8239u;
const GL_RG16UI                         : GLenum = 0x823Au;
const GL_RG32I                          : GLenum = 0x823Bu;
const GL_RG32UI                         : GLenum = 0x823Cu;

const GL_VERTEX_ARRAY_BINDING           : GLenum = 0x85B5u;

const GL_UNIFORM_BUFFER                 : GLenum = 0x8A11u;
const GL_UNIFORM_BUFFER_BINDING         : GLenum = 0x8A28u;
const GL_UNIFORM_BUFFER_START           : GLenum = 0x8A29u;
const GL_UNIFORM_BUFFER_SIZE            : GLenum = 0x8A2Au;
const GL_MAX_VERTEX_UNIFORM_BLOCKS      : GLenum = 0x8A2Bu;
const GL_MAX_GEOMETRY_UNIFORM_BLOCKS    : GLenum = 0x8A2Cu;
const GL_MAX_FRAGMENT_UNIFORM_BLOCKS    : GLenum = 0x8A2Du;
const GL_MAX_COMBINED_UNIFORM_BLOCKS    : GLenum = 0x8A2Eu;
const GL_MAX_UNIFORM_BUFFER_BINDINGS    : GLenum = 0x8A2Fu;
const GL_MAX_UNIFORM_BLOCK_SIZE         : GLenum = 0x8A30u;
const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS : GLenum = 0x8A31u;
const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS : GLenum = 0x8A32u;
const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS : GLenum = 0x8A33u;
const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT : GLenum = 0x8A34u;
const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH : GLenum = 0x8A35u;
const GL_ACTIVE_UNIFORM_BLOCKS          : GLenum = 0x8A36u;
const GL_UNIFORM_TYPE                   : GLenum = 0x8A37u;
const GL_UNIFORM_SIZE                   : GLenum = 0x8A38u;
const GL_UNIFORM_NAME_LENGTH            : GLenum = 0x8A39u;
const GL_UNIFORM_BLOCK_INDEX            : GLenum = 0x8A3Au;
const GL_UNIFORM_OFFSET                 : GLenum = 0x8A3Bu;
const GL_UNIFORM_ARRAY_STRIDE           : GLenum = 0x8A3Cu;
const GL_UNIFORM_MATRIX_STRIDE          : GLenum = 0x8A3Du;
const GL_UNIFORM_IS_ROW_MAJOR           : GLenum = 0x8A3Eu;
const GL_UNIFORM_BLOCK_BINDING          : GLenum = 0x8A3Fu;
const GL_UNIFORM_BLOCK_DATA_SIZE        : GLenum = 0x8A40u;
const GL_UNIFORM_BLOCK_NAME_LENGTH      : GLenum = 0x8A41u;
const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS  : GLenum = 0x8A42u;
const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES : GLenum = 0x8A43u;
const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER : GLenum = 0x8A44u;
const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER : GLenum = 0x8A45u;
const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER : GLenum = 0x8A46u;
const GL_INVALID_INDEX                  : GLenum = 0xFFFFFFFFu;

const GL_COPY_READ_BUFFER               : GLenum = 0x8F36u;
const GL_COPY_WRITE_BUFFER              : GLenum = 0x8F37u;

const GL_DEPTH_CLAMP                    : GLenum = 0x864Fu;

const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION : GLenum = 0x8E4Cu;
const GL_FIRST_VERTEX_CONVENTION        : GLenum = 0x8E4Du;
const GL_LAST_VERTEX_CONVENTION         : GLenum = 0x8E4Eu;
const GL_PROVOKING_VERTEX               : GLenum = 0x8E4Fu;

const GL_TEXTURE_CUBE_MAP_SEAMLESS      : GLenum = 0x884Fu;

const GL_MAX_SERVER_WAIT_TIMEOUT        : GLenum = 0x9111u;
const GL_OBJECT_TYPE                    : GLenum = 0x9112u;
const GL_SYNC_CONDITION                 : GLenum = 0x9113u;
const GL_SYNC_STATUS                    : GLenum = 0x9114u;
const GL_SYNC_FLAGS                     : GLenum = 0x9115u;
const GL_SYNC_FENCE                     : GLenum = 0x9116u;
const GL_SYNC_GPU_COMMANDS_COMPLETE     : GLenum = 0x9117u;
const GL_UNSIGNALED                     : GLenum = 0x9118u;
const GL_SIGNALED                       : GLenum = 0x9119u;
const GL_ALREADY_SIGNALED               : GLenum = 0x911Au;
const GL_TIMEOUT_EXPIRED                : GLenum = 0x911Bu;
const GL_CONDITION_SATISFIED            : GLenum = 0x911Cu;
const GL_WAIT_FAILED                    : GLenum = 0x911Du;
const GL_SYNC_FLUSH_COMMANDS_BIT        : GLenum = 0x00000001u;
const GL_TIMEOUT_IGNORED                : GLenum = 0xFFFFFFFFu;

const GL_SAMPLE_POSITION                : GLenum = 0x8E50u;
const GL_SAMPLE_MASK                    : GLenum = 0x8E51u;
const GL_SAMPLE_MASK_VALUE              : GLenum = 0x8E52u;
const GL_MAX_SAMPLE_MASK_WORDS          : GLenum = 0x8E59u;
const GL_TEXTURE_2D_MULTISAMPLE         : GLenum = 0x9100u;
const GL_PROXY_TEXTURE_2D_MULTISAMPLE   : GLenum = 0x9101u;
const GL_TEXTURE_2D_MULTISAMPLE_ARRAY   : GLenum = 0x9102u;
const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY : GLenum = 0x9103u;
const GL_TEXTURE_BINDING_2D_MULTISAMPLE : GLenum = 0x9104u;
const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY : GLenum = 0x9105u;
const GL_TEXTURE_SAMPLES                : GLenum = 0x9106u;
const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS : GLenum = 0x9107u;
const GL_SAMPLER_2D_MULTISAMPLE         : GLenum = 0x9108u;
const GL_INT_SAMPLER_2D_MULTISAMPLE     : GLenum = 0x9109u;
const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE : GLenum = 0x910Au;
const GL_SAMPLER_2D_MULTISAMPLE_ARRAY   : GLenum = 0x910Bu;
const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY : GLenum = 0x910Cu;
const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY : GLenum = 0x910Du;
const GL_MAX_COLOR_TEXTURE_SAMPLES      : GLenum = 0x910Eu;
const GL_MAX_DEPTH_TEXTURE_SAMPLES      : GLenum = 0x910Fu;
const GL_MAX_INTEGER_SAMPLES            : GLenum = 0x9110u;

const GL_SAMPLE_SHADING_ARB             : GLenum = 0x8C36u;
const GL_MIN_SAMPLE_SHADING_VALUE_ARB   : GLenum = 0x8C37u;

const GL_TEXTURE_CUBE_MAP_ARRAY_ARB     : GLenum = 0x9009u;
const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB : GLenum = 0x900Au;
const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB : GLenum = 0x900Bu;
const GL_SAMPLER_CUBE_MAP_ARRAY_ARB     : GLenum = 0x900Cu;
const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB : GLenum = 0x900Du;
const GL_INT_SAMPLER_CUBE_MAP_ARRAY_ARB : GLenum = 0x900Eu;
const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB : GLenum = 0x900Fu;

const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET_ARB : GLenum = 0x8E5Eu;
const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET_ARB : GLenum = 0x8E5Fu;

const GL_SHADER_INCLUDE_ARB             : GLenum = 0x8DAEu;
const GL_NAMED_STRING_LENGTH_ARB        : GLenum = 0x8DE9u;
const GL_NAMED_STRING_TYPE_ARB          : GLenum = 0x8DEAu;

const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB : GLenum = 0x8E8Cu;
const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB : GLenum = 0x8E8Du;
const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB : GLenum = 0x8E8Eu;
const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB : GLenum = 0x8E8Fu;

const GL_SRC1_COLOR                     : GLenum = 0x88F9u;

const GL_ONE_MINUS_SRC1_COLOR           : GLenum = 0x88FAu;
const GL_ONE_MINUS_SRC1_ALPHA           : GLenum = 0x88FBu;
const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS   : GLenum = 0x88FCu;

const GL_ANY_SAMPLES_PASSED             : GLenum = 0x8C2Fu;

const GL_SAMPLER_BINDING                : GLenum = 0x8919u;

const GL_RGB10_A2UI                     : GLenum = 0x906Fu;

const GL_TEXTURE_SWIZZLE_R              : GLenum = 0x8E42u;
const GL_TEXTURE_SWIZZLE_G              : GLenum = 0x8E43u;
const GL_TEXTURE_SWIZZLE_B              : GLenum = 0x8E44u;
const GL_TEXTURE_SWIZZLE_A              : GLenum = 0x8E45u;
const GL_TEXTURE_SWIZZLE_RGBA           : GLenum = 0x8E46u;

const GL_TIME_ELAPSED                   : GLenum = 0x88BFu;
const GL_TIMESTAMP                      : GLenum = 0x8E28u;

const GL_INT_2_10_10_10_REV             : GLenum = 0x8D9Fu;

const GL_DRAW_INDIRECT_BUFFER           : GLenum = 0x8F3Fu;
const GL_DRAW_INDIRECT_BUFFER_BINDING   : GLenum = 0x8F43u;

const GL_GEOMETRY_SHADER_INVOCATIONS    : GLenum = 0x887Fu;
const GL_MAX_GEOMETRY_SHADER_INVOCATIONS : GLenum = 0x8E5Au;
const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET : GLenum = 0x8E5Bu;
const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET : GLenum = 0x8E5Cu;
const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS : GLenum = 0x8E5Du;

const GL_DOUBLE_VEC2                    : GLenum = 0x8FFCu;
const GL_DOUBLE_VEC3                    : GLenum = 0x8FFDu;
const GL_DOUBLE_VEC4                    : GLenum = 0x8FFEu;
const GL_DOUBLE_MAT2                    : GLenum = 0x8F46u;
const GL_DOUBLE_MAT3                    : GLenum = 0x8F47u;
const GL_DOUBLE_MAT4                    : GLenum = 0x8F48u;
const GL_DOUBLE_MAT2x3                  : GLenum = 0x8F49u;
const GL_DOUBLE_MAT2x4                  : GLenum = 0x8F4Au;
const GL_DOUBLE_MAT3x2                  : GLenum = 0x8F4Bu;
const GL_DOUBLE_MAT3x4                  : GLenum = 0x8F4Cu;
const GL_DOUBLE_MAT4x2                  : GLenum = 0x8F4Du;
const GL_DOUBLE_MAT4x3                  : GLenum = 0x8F4Eu;

const GL_ACTIVE_SUBROUTINES             : GLenum = 0x8DE5u;
const GL_ACTIVE_SUBROUTINE_UNIFORMS     : GLenum = 0x8DE6u;
const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS : GLenum = 0x8E47u;
const GL_ACTIVE_SUBROUTINE_MAX_LENGTH   : GLenum = 0x8E48u;
const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH : GLenum = 0x8E49u;
const GL_MAX_SUBROUTINES                : GLenum = 0x8DE7u;
const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS : GLenum = 0x8DE8u;
const GL_NUM_COMPATIBLE_SUBROUTINES     : GLenum = 0x8E4Au;
const GL_COMPATIBLE_SUBROUTINES         : GLenum = 0x8E4Bu;

const GL_PATCHES                        : GLenum = 0x000Eu;
const GL_PATCH_VERTICES                 : GLenum = 0x8E72u;
const GL_PATCH_DEFAULT_INNER_LEVEL      : GLenum = 0x8E73u;
const GL_PATCH_DEFAULT_OUTER_LEVEL      : GLenum = 0x8E74u;
const GL_TESS_CONTROL_OUTPUT_VERTICES   : GLenum = 0x8E75u;
const GL_TESS_GEN_MODE                  : GLenum = 0x8E76u;
const GL_TESS_GEN_SPACING               : GLenum = 0x8E77u;
const GL_TESS_GEN_VERTEX_ORDER          : GLenum = 0x8E78u;
const GL_TESS_GEN_POINT_MODE            : GLenum = 0x8E79u;

const GL_ISOLINES                       : GLenum = 0x8E7Au;

const GL_FRACTIONAL_ODD                 : GLenum = 0x8E7Bu;
const GL_FRACTIONAL_EVEN                : GLenum = 0x8E7Cu;

const GL_MAX_PATCH_VERTICES             : GLenum = 0x8E7Du;
const GL_MAX_TESS_GEN_LEVEL             : GLenum = 0x8E7Eu;
const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS : GLenum = 0x8E7Fu;
const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS : GLenum = 0x8E80u;
const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS : GLenum = 0x8E81u;
const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS : GLenum = 0x8E82u;
const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS : GLenum = 0x8E83u;
const GL_MAX_TESS_PATCH_COMPONENTS      : GLenum = 0x8E84u;
const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS : GLenum = 0x8E85u;
const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS : GLenum = 0x8E86u;
const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS : GLenum = 0x8E89u;
const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS : GLenum = 0x8E8Au;
const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS : GLenum = 0x886Cu;
const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS : GLenum = 0x886Du;
const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS : GLenum = 0x8E1Eu;
const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS : GLenum = 0x8E1Fu;
const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER : GLenum = 0x84F0u;
const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER : GLenum = 0x84F1u;
const GL_TESS_EVALUATION_SHADER         : GLenum = 0x8E87u;
const GL_TESS_CONTROL_SHADER            : GLenum = 0x8E88u;

const GL_TRANSFORM_FEEDBACK             : GLenum = 0x8E22u;
const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED : GLenum = 0x8E23u;
const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE : GLenum = 0x8E24u;
const GL_TRANSFORM_FEEDBACK_BINDING     : GLenum = 0x8E25u;

const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS : GLenum = 0x8E70u;
const GL_MAX_VERTEX_STREAMS             : GLenum = 0x8E71u;

const GL_FIXED                          : GLenum = 0x140Cu;
const GL_IMPLEMENTATION_COLOR_READ_TYPE : GLenum = 0x8B9Au;
const GL_IMPLEMENTATION_COLOR_READ_FORMAT : GLenum = 0x8B9Bu;
const GL_LOW_FLOAT                      : GLenum = 0x8DF0u;
const GL_MEDIUM_FLOAT                   : GLenum = 0x8DF1u;
const GL_HIGH_FLOAT                     : GLenum = 0x8DF2u;
const GL_LOW_INT                        : GLenum = 0x8DF3u;
const GL_MEDIUM_INT                     : GLenum = 0x8DF4u;
const GL_HIGH_INT                       : GLenum = 0x8DF5u;
const GL_SHADER_COMPILER                : GLenum = 0x8DFAu;
const GL_NUM_SHADER_BINARY_FORMATS      : GLenum = 0x8DF9u;
const GL_MAX_VERTEX_UNIFORM_VECTORS     : GLenum = 0x8DFBu;
const GL_MAX_VARYING_VECTORS            : GLenum = 0x8DFCu;
const GL_MAX_FRAGMENT_UNIFORM_VECTORS   : GLenum = 0x8DFDu;

const GL_PROGRAM_BINARY_RETRIEVABLE_HINT : GLenum = 0x8257u;
const GL_PROGRAM_BINARY_LENGTH          : GLenum = 0x8741u;
const GL_NUM_PROGRAM_BINARY_FORMATS     : GLenum = 0x87FEu;
const GL_PROGRAM_BINARY_FORMATS         : GLenum = 0x87FFu;

const GL_VERTEX_SHADER_BIT              : GLenum = 0x00000001u;
const GL_FRAGMENT_SHADER_BIT            : GLenum = 0x00000002u;
const GL_GEOMETRY_SHADER_BIT            : GLenum = 0x00000004u;
const GL_TESS_CONTROL_SHADER_BIT        : GLenum = 0x00000008u;
const GL_TESS_EVALUATION_SHADER_BIT     : GLenum = 0x00000010u;
const GL_ALL_SHADER_BITS                : GLenum = 0xFFFFFFFFu;
const GL_PROGRAM_SEPARABLE              : GLenum = 0x8258u;
const GL_ACTIVE_PROGRAM                 : GLenum = 0x8259u;
const GL_PROGRAM_PIPELINE_BINDING       : GLenum = 0x825Au;

const GL_MAX_VIEWPORTS                  : GLenum = 0x825Bu;
const GL_VIEWPORT_SUBPIXEL_BITS         : GLenum = 0x825Cu;
const GL_VIEWPORT_BOUNDS_RANGE          : GLenum = 0x825Du;
const GL_LAYER_PROVOKING_VERTEX         : GLenum = 0x825Eu;
const GL_VIEWPORT_INDEX_PROVOKING_VERTEX : GLenum = 0x825Fu;
const GL_UNDEFINED_VERTEX               : GLenum = 0x8260u;

const GL_SYNC_CL_EVENT_ARB              : GLenum = 0x8240u;
const GL_SYNC_CL_EVENT_COMPLETE_ARB     : GLenum = 0x8241u;

const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB   : GLenum = 0x8242u;
const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB : GLenum = 0x8243u;
const GL_DEBUG_CALLBACK_FUNCTION_ARB    : GLenum = 0x8244u;
const GL_DEBUG_CALLBACK_USER_PARAM_ARB  : GLenum = 0x8245u;
const GL_DEBUG_SOURCE_API_ARB           : GLenum = 0x8246u;
const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB : GLenum = 0x8247u;
const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB : GLenum = 0x8248u;
const GL_DEBUG_SOURCE_THIRD_PARTY_ARB   : GLenum = 0x8249u;
const GL_DEBUG_SOURCE_APPLICATION_ARB   : GLenum = 0x824Au;
const GL_DEBUG_SOURCE_OTHER_ARB         : GLenum = 0x824Bu;
const GL_DEBUG_TYPE_ERROR_ARB           : GLenum = 0x824Cu;
const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB : GLenum = 0x824Du;
const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB : GLenum = 0x824Eu;
const GL_DEBUG_TYPE_PORTABILITY_ARB     : GLenum = 0x824Fu;
const GL_DEBUG_TYPE_PERFORMANCE_ARB     : GLenum = 0x8250u;
const GL_DEBUG_TYPE_OTHER_ARB           : GLenum = 0x8251u;
const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB   : GLenum = 0x9143u;
const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB  : GLenum = 0x9144u;
const GL_DEBUG_LOGGED_MESSAGES_ARB      : GLenum = 0x9145u;
const GL_DEBUG_SEVERITY_HIGH_ARB        : GLenum = 0x9146u;
const GL_DEBUG_SEVERITY_MEDIUM_ARB      : GLenum = 0x9147u;
const GL_DEBUG_SEVERITY_LOW_ARB         : GLenum = 0x9148u;

const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB : GLenum = 0x00000004u;
const GL_LOSE_CONTEXT_ON_RESET_ARB      : GLenum = 0x8252u;
const GL_GUILTY_CONTEXT_RESET_ARB       : GLenum = 0x8253u;
const GL_INNOCENT_CONTEXT_RESET_ARB     : GLenum = 0x8254u;
const GL_UNKNOWN_CONTEXT_RESET_ARB      : GLenum = 0x8255u;
const GL_RESET_NOTIFICATION_STRATEGY_ARB : GLenum = 0x8256u;
const GL_NO_RESET_NOTIFICATION_ARB      : GLenum = 0x8261u;

const GL_UNPACK_COMPRESSED_BLOCK_WIDTH  : GLenum = 0x9127u;
const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT : GLenum = 0x9128u;
const GL_UNPACK_COMPRESSED_BLOCK_DEPTH  : GLenum = 0x9129u;
const GL_UNPACK_COMPRESSED_BLOCK_SIZE   : GLenum = 0x912Au;
const GL_PACK_COMPRESSED_BLOCK_WIDTH    : GLenum = 0x912Bu;
const GL_PACK_COMPRESSED_BLOCK_HEIGHT   : GLenum = 0x912Cu;
const GL_PACK_COMPRESSED_BLOCK_DEPTH    : GLenum = 0x912Du;
const GL_PACK_COMPRESSED_BLOCK_SIZE     : GLenum = 0x912Eu;

const GL_NUM_SAMPLE_COUNTS              : GLenum = 0x9380u;

const GL_MIN_MAP_BUFFER_ALIGNMENT       : GLenum = 0x90BCu;

const GL_ATOMIC_COUNTER_BUFFER          : GLenum = 0x92C0u;
const GL_ATOMIC_COUNTER_BUFFER_BINDING  : GLenum = 0x92C1u;
const GL_ATOMIC_COUNTER_BUFFER_START    : GLenum = 0x92C2u;
const GL_ATOMIC_COUNTER_BUFFER_SIZE     : GLenum = 0x92C3u;
const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE : GLenum = 0x92C4u;
const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS : GLenum = 0x92C5u;
const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES : GLenum = 0x92C6u;
const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER : GLenum = 0x92C7u;
const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER : GLenum = 0x92C8u;
const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER : GLenum = 0x92C9u;
const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER : GLenum = 0x92CAu;
const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER : GLenum = 0x92CBu;
const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CCu;
const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CDu;
const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CEu;
const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CFu;
const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92D0u;
const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92D1u;
const GL_MAX_VERTEX_ATOMIC_COUNTERS     : GLenum = 0x92D2u;
const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS : GLenum = 0x92D3u;
const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS : GLenum = 0x92D4u;
const GL_MAX_GEOMETRY_ATOMIC_COUNTERS   : GLenum = 0x92D5u;
const GL_MAX_FRAGMENT_ATOMIC_COUNTERS   : GLenum = 0x92D6u;
const GL_MAX_COMBINED_ATOMIC_COUNTERS   : GLenum = 0x92D7u;
const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE : GLenum = 0x92D8u;
const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS : GLenum = 0x92DCu;
const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS  : GLenum = 0x92D9u;
const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX : GLenum = 0x92DAu;
const GL_UNSIGNED_INT_ATOMIC_COUNTER    : GLenum = 0x92DBu;

const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT : GLenum = 0x00000001u;
const GL_ELEMENT_ARRAY_BARRIER_BIT      : GLenum = 0x00000002u;
const GL_UNIFORM_BARRIER_BIT            : GLenum = 0x00000004u;
const GL_TEXTURE_FETCH_BARRIER_BIT      : GLenum = 0x00000008u;
const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT : GLenum = 0x00000020u;
const GL_COMMAND_BARRIER_BIT            : GLenum = 0x00000040u;
const GL_PIXEL_BUFFER_BARRIER_BIT       : GLenum = 0x00000080u;
const GL_TEXTURE_UPDATE_BARRIER_BIT     : GLenum = 0x00000100u;
const GL_BUFFER_UPDATE_BARRIER_BIT      : GLenum = 0x00000200u;
const GL_FRAMEBUFFER_BARRIER_BIT        : GLenum = 0x00000400u;
const GL_TRANSFORM_FEEDBACK_BARRIER_BIT : GLenum = 0x00000800u;
const GL_ATOMIC_COUNTER_BARRIER_BIT     : GLenum = 0x00001000u;
const GL_ALL_BARRIER_BITS               : GLenum = 0xFFFFFFFFu;
const GL_MAX_IMAGE_UNITS                : GLenum = 0x8F38u;
const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS : GLenum = 0x8F39u;
const GL_IMAGE_BINDING_NAME             : GLenum = 0x8F3Au;
const GL_IMAGE_BINDING_LEVEL            : GLenum = 0x8F3Bu;
const GL_IMAGE_BINDING_LAYERED          : GLenum = 0x8F3Cu;
const GL_IMAGE_BINDING_LAYER            : GLenum = 0x8F3Du;
const GL_IMAGE_BINDING_ACCESS           : GLenum = 0x8F3Eu;
const GL_IMAGE_1D                       : GLenum = 0x904Cu;
const GL_IMAGE_2D                       : GLenum = 0x904Du;
const GL_IMAGE_3D                       : GLenum = 0x904Eu;
const GL_IMAGE_2D_RECT                  : GLenum = 0x904Fu;
const GL_IMAGE_CUBE                     : GLenum = 0x9050u;
const GL_IMAGE_BUFFER                   : GLenum = 0x9051u;
const GL_IMAGE_1D_ARRAY                 : GLenum = 0x9052u;
const GL_IMAGE_2D_ARRAY                 : GLenum = 0x9053u;
const GL_IMAGE_CUBE_MAP_ARRAY           : GLenum = 0x9054u;
const GL_IMAGE_2D_MULTISAMPLE           : GLenum = 0x9055u;
const GL_IMAGE_2D_MULTISAMPLE_ARRAY     : GLenum = 0x9056u;
const GL_INT_IMAGE_1D                   : GLenum = 0x9057u;
const GL_INT_IMAGE_2D                   : GLenum = 0x9058u;
const GL_INT_IMAGE_3D                   : GLenum = 0x9059u;
const GL_INT_IMAGE_2D_RECT              : GLenum = 0x905Au;
const GL_INT_IMAGE_CUBE                 : GLenum = 0x905Bu;
const GL_INT_IMAGE_BUFFER               : GLenum = 0x905Cu;
const GL_INT_IMAGE_1D_ARRAY             : GLenum = 0x905Du;
const GL_INT_IMAGE_2D_ARRAY             : GLenum = 0x905Eu;
const GL_INT_IMAGE_CUBE_MAP_ARRAY       : GLenum = 0x905Fu;
const GL_INT_IMAGE_2D_MULTISAMPLE       : GLenum = 0x9060u;
const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY : GLenum = 0x9061u;
const GL_UNSIGNED_INT_IMAGE_1D          : GLenum = 0x9062u;
const GL_UNSIGNED_INT_IMAGE_2D          : GLenum = 0x9063u;
const GL_UNSIGNED_INT_IMAGE_3D          : GLenum = 0x9064u;
const GL_UNSIGNED_INT_IMAGE_2D_RECT     : GLenum = 0x9065u;
const GL_UNSIGNED_INT_IMAGE_CUBE        : GLenum = 0x9066u;
const GL_UNSIGNED_INT_IMAGE_BUFFER      : GLenum = 0x9067u;
const GL_UNSIGNED_INT_IMAGE_1D_ARRAY    : GLenum = 0x9068u;
const GL_UNSIGNED_INT_IMAGE_2D_ARRAY    : GLenum = 0x9069u;
const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY : GLenum = 0x906Au;
const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE : GLenum = 0x906Bu;
const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY : GLenum = 0x906Cu;
const GL_MAX_IMAGE_SAMPLES              : GLenum = 0x906Du;
const GL_IMAGE_BINDING_FORMAT           : GLenum = 0x906Eu;
const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE : GLenum = 0x90C7u;
const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE : GLenum = 0x90C8u;
const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS : GLenum = 0x90C9u;
const GL_MAX_VERTEX_IMAGE_UNIFORMS      : GLenum = 0x90CAu;
const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS : GLenum = 0x90CBu;
const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS : GLenum = 0x90CCu;
const GL_MAX_GEOMETRY_IMAGE_UNIFORMS    : GLenum = 0x90CDu;
const GL_MAX_FRAGMENT_IMAGE_UNIFORMS    : GLenum = 0x90CEu;
const GL_MAX_COMBINED_IMAGE_UNIFORMS    : GLenum = 0x90CFu;

const GL_TEXTURE_IMMUTABLE_FORMAT       : GLenum = 0x912Fu;



/*************************************************************/

#[nolink]
extern mod GL {
    
    /******** A ********/
    
    fn glActiveShaderProgram (pipeline : GLuint, program : GLuint);
    fn glActiveTexture (texture : GLenum);
    fn glAttachShader(program : GLuint,  shader : GLuint);
    
    
    /******** B ********/
    
    fn glBeginConditionalRender(id : GLuint, mode : GLenum);
    fn glBeginQuery(target : GLenum, id : GLuint);
    fn glBeginQueryIndexed(target : GLenum, index : GLuint, id : GLuint);
    fn glBeginTransformFeedback(primitiveMode : GLenum);
    fn glBindAttribLocation(program : GLuint, index : GLuint, name : *GLchar);
    fn glBindBuffer(target : GLenum, buffer : GLuint);
    fn glBindBufferBase(target : GLenum, index : GLuint, buffer : GLuint);
    fn glBindBufferRange(target : GLenum, index : GLuint, buffer : GLuint, offset : GLintptr, size : GLsizeiptr);
    fn glBindFragDataLocation(program : GLuint, colorNumber : GLuint, name : *u8);
    fn glBindFragDataLocationIndexed(program : GLuint, colorNumber : GLuint, index : GLuint, name : *u8);
    fn glBindFramebuffer(target : GLenum, framebuffer : GLuint);
    fn glBindImageTexture(unit : GLuint, texture : GLuint, level : GLint, layered : GLboolean, layer : GLint, access : GLenum, format : GLenum);
    fn glBindProgramPipeline(pipeline : GLuint);
    fn glBindRenderbuffer(target : GLenum, renderbuffer : GLuint);
    fn glBindSampler(unit : GLuint, sampler : GLuint);
    fn glBindTexture(target : GLenum, texture : GLuint);
    fn glBindTransformFeedback(target : GLenum, id : GLuint);
    fn glBindVertexArray(array : GLuint);
    fn glBlendColor(red : GLclampf, green : GLclampf, blue : GLclampf, alpha : GLclampf);
    fn glBlendEquation(mode : GLenum);
    fn glBlendEquationi(buf : GLuint, mode : GLenum);
    fn glBlendEquationSeparate(modeRGB : GLenum, modeAlpha : GLenum);
    fn glBlendEquationSeparatei(buf : GLuint, modeRGB : GLenum, modeAlpha : GLenum);
    fn glBlendFunc(sfactor : GLenum, dfactor : GLenum);
    fn glBlendFunci(buf : GLuint, sfactor : GLenum, dfactor : GLenum);
    fn glBlendFuncSeparate(srcRGB : GLenum, dstRGB :  GLenum, srcAlpha : GLenum, dstAlpha : GLenum);
    fn glBlendFuncSeparatei(buf : GLuint, srcRGB : GLenum, dstRGB : GLenum, srcAlpha : GLenum, dstAlpha : GLenum);
    fn glBlitFramebuffer(srcX0 : GLint, srcY0 : GLint, srcX1 : GLint, srcY1 : GLint, dstX0 : GLint, dstY0 : GLint, dstX1 : GLint, dstY1 : GLint, mask : GLbitfield, filter : GLenum);
    fn glBufferData(target : GLenum, size : GLsizeiptr, data : *uint, usage : GLenum);
    fn glBufferSubData(target : GLenum, offset : GLintptr, size : GLsizeiptr, data : *uint);
    
    
    /******** C ********/
    
    fn glCheckFramebufferStatus(target : GLenum) -> GLenum;
    fn glClampColor(target : GLenum, clamp : GLenum);
    fn glClear(mask : GLbitfield);
    fn glClearBufferiv(buffer : GLenum, drawBuffer : GLint, value : *GLint);
    fn glClearBufferuiv(buffer : GLenum, drawBuffer : GLint, value : *GLuint);
    fn glClearBufferfv(buffer : GLenum, drawBuffer : GLint,  value : *GLfloat);
    fn glClearBufferfi(buffer : GLenum, drawBuffer : GLint,  depth : GLfloat, stencil : GLint);
    fn glClearColor(red : GLclampf, green : GLclampf, blue : GLclampf, alpha : GLclampf);
    fn glClearDepth(depth : GLclampd);
    fn glClearDepthf(depth : GLclampf);
    fn glClearStencil(s : GLint);
    fn glClientWaitSync(sync : GLsync, flags : GLbitfield, timeout : GLuint64) -> GLenum;
    fn glColorMask(red : GLboolean, green : GLboolean, blue : GLboolean, alpha : GLboolean);
    fn glColorMaski(buf : GLuint, red : GLboolean, green : GLboolean, blue : GLboolean, alpha : GLboolean);
    fn glCompileShader(shader : GLuint);
    fn glCompressedTexImage1D(target : GLenum, level : GLint, internalformat : GLenum, width : GLsizei, border : GLint, imageSize : GLsizei, data : *uint);
    fn glCompressedTexImage2D(target : GLenum, level : GLint, internalformat : GLenum, width : GLsizei, height : GLsizei, border : GLint, imageSize : GLsizei, data : *uint);
    fn glCompressedTexImage3D(target : GLenum, level : GLint, internalformat : GLenum, width : GLsizei, height : GLsizei, depth : GLsizei, border : GLint, imageSize : GLsizei, data : *uint);
    fn glCompressedTexSubImage1D(target : GLenum, level : GLint, xoffset : GLint, width : GLsizei, format : GLenum, imageSize : GLsizei, data : *uint);
    fn glCompressedTexSubImage2D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, width : GLsizei, height : GLsizei, format : GLenum, imageSize : GLsizei, data : *uint);
    fn glCompressedTexSubImage3D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, zoffset : GLint, width : GLsizei, height : GLsizei, depth : GLsizei, format : GLenum, imageSize : GLsizei, data : *uint);
    fn glCopyBufferSubData(readtarget : GLenum, writetarget : GLenum, readoffset : GLintptr, writeoffset : GLintptr, size : GLsizeiptr);
    fn glCopyTexImage1D(target : GLenum, level : GLint, internalformat : GLenum, x : GLint, y : GLint, width : GLsizei, border : GLint);
    fn glCopyTexImage2D(target : GLenum, level : GLint, internalformat : GLenum, x : GLint, y : GLint, width : GLsizei, height : GLsizei, border : GLint);
    fn glCopyTexSubImage1D(target : GLenum, level : GLint, xoffset : GLint, x : GLint, y : GLint, width : GLsizei);
    fn glCopyTexSubImage2D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, x : GLint, y : GLint, width : GLsizei, height : GLsizei);
    fn glCopyTexSubImage3D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, zoffset : GLint, x : GLint, y : GLint, width : GLsizei, height : GLsizei);
    fn glCreateProgram() -> GLuint;
    fn glCreateShader(shaderType : GLenum) -> GLuint;
    fn glCreateShaderProgramv(shaderType : GLenum, count : GLsizei, strings : **GLchar) -> GLuint;
    fn glCullFace(mode : GLenum);
    
    
    /******** D ********/
    
    fn glDeleteBuffers(n : GLsizei, buffers : *GLuint);
    fn glDeleteFramebuffers(n : GLsizei, framebuffers : *GLuint);
    fn glDeleteProgram(program : GLuint);
    fn glDeleteProgramPipelines(n : GLsizei, pipelines : *GLuint);
    fn glDeleteQueries(n : GLsizei, ids : *GLuint);
    fn glDeleteRenderbuffers(n : GLsizei, renderbuffers : *GLuint);
    fn glDeleteSamplers(n : GLsizei, ids : *GLuint);
    fn glDeleteShader(shader : GLuint);
    fn glDeleteSync(sync : GLsync);
    fn glDeleteTextures(n : GLsizei, textures : *GLuint);
    fn glDeleteTransformFeedbacks(n : GLsizei, ids : *GLuint);
    fn glDeleteVertexArrays(n : GLsizei, arrays : *GLuint);
    fn glDepthFunc(func : GLenum);
    fn glDepthMask(flag : GLboolean);
    fn glDepthRange(nearVal : GLclampd, farVal : GLclampd);
    fn glDepthRangef(nearVal : GLclampf, farVal : GLclampf);
    fn glDepthRangeArrayv(first : GLuint, count : GLsizei, v : *GLclampd);
    fn glDepthRangeIndexed(index : GLuint, nearVal : GLclampd, farVal : GLclampd);
    fn glDetachShader(program : GLuint, shader : GLuint);
    fn glDisable(cap : GLenum);
    fn glDisablei(cap : GLenum, index : GLuint);
    fn glDisableVertexAttribArray(index : GLuint);
    fn glDrawArrays(mode : GLenum, first : GLint, count : GLsizei);
    fn glDrawArraysIndirect(mode : GLenum, indirect : *uint);
    fn glDrawArraysInstanced(mode : GLenum, first : GLint, count : GLsizei, primcount : GLsizei);
    fn glDrawArraysInstancedBaseInstance(mode : GLenum, first : GLint, count : GLsizei, primcount : GLsizei, baseinstance : GLuint);
    fn glDrawBuffer(mode : GLenum);
    fn glDrawBuffers(n : GLsizei, bufs : *GLenum);
    fn glDrawElements(mode : GLenum, count : GLsizei, etype : GLenum, indices : *uint);
    fn glDrawElementsBaseVertex(mode : GLenum, count : GLsizei, etype : GLenum, indices : *uint, basevertex : GLint);
    fn glDrawElementsIndirect(mode : GLenum, etype : GLenum, indirect : *uint);
    fn glDrawElementsInstanced(mode : GLenum, count : GLsizei, etype : GLenum, indices : *uint, primcount : GLsizei);
    fn glDrawElementsInstancedBaseInstance(mode : GLenum, count : GLsizei, etype : GLenum, indices : *uint, primcount : GLsizei, baseinstance : GLuint);
    fn glDrawElementsInstancedBaseVertex(mode : GLenum, count : GLsizei, etype : GLenum, indices : *uint, primcount : GLsizei,  basevertex : GLint);
    fn glDrawElementsInstancedBaseVertexBaseInstance(mode : GLenum, count : GLsizei, etype : GLenum, indices : *uint, primcount : GLsizei, basevertex : GLint, baseinstance : GLuint);
    fn glDrawRangeElements(mode : GLenum, start : GLuint, end : GLuint, count : GLsizei, etype : GLenum, indices : *uint);
    fn glDrawRangeElementsBaseVertex(mode : GLenum, start : GLuint, end : GLuint, count : GLsizei, etype : GLenum, indices : *uint, basevertex : GLint);
    fn glDrawTransformFeedback(mode : GLenum, id : GLuint);
    fn glDrawTransformFeedbackInstanced(mode : GLenum, id : GLuint, primcount : GLsizei);
    fn glDrawTransformFeedbackStream(mode : GLenum, id : GLuint, stream : GLuint);
    fn glDrawTransformFeedbackStreamInstanced(mode : GLenum, id : GLuint, stream : GLuint, primcount : GLsizei);
    
    
    /******** E ********/
    
    fn glEnable(cap : GLenum);
    fn glEnablei(cap : GLenum, index : GLuint);
    fn glEnableVertexAttribArray(index : GLuint);
    fn glEndConditionalRender();
    fn glEndQuery(target : GLenum);
    fn glEndQueryIndexed(target : GLenum, index : GLuint);
    fn glEndTransformFeedback();
    
    
    /******** F ********/
    
    fn glFenceSync(condition : GLenum, flags : GLbitfield) -> GLsync;
    fn glFinish();
    fn glFlush();
    fn glFlushMappedBufferRange(target : GLenum, offset : GLintptr, length : GLsizeiptr) -> GLsync;
    fn glFramebufferRenderbuffer(target : GLenum, attachment : GLenum, renderbuffertarget : GLenum, renderbuffer : GLuint) -> GLsync;
    fn glFramebufferTexture(target : GLenum, attachment : GLenum, texture : GLuint, level : GLint);
    fn glFramebufferTexture1D(target : GLenum, attachment : GLenum, textarget : GLenum, texture : GLuint, level : GLint);
    fn glFramebufferTexture2D(target : GLenum, attachment : GLenum, textarget : GLenum, texture : GLuint, level : GLint);
    fn glFramebufferTexture3D(target : GLenum, attachment : GLenum, textarget : GLenum, texture : GLuint, level : GLint, layer : GLint);
    fn glFramebufferTextureLayer(target : GLenum, attachment : GLenum, texture : GLuint, level : GLint, layer : GLint);
    fn glFrontFace(mode : GLenum);
    
    
    /******** G ********/
    
    fn glGenBuffers(n : GLsizei, buffers : *GLuint);
    fn glGenFramebuffers(n : GLsizei, ids : *GLuint);
    fn glGenProgramPipelines(n : GLsizei, pipelines : *GLuint);
    fn glGenQueries(n : GLsizei, ids : *GLuint);
    fn glGenRenderbuffers(n : GLsizei, renderbuffers : *GLuint);
    fn glGenSamplers(n : GLsizei, samplers : *GLuint);
    fn glGenTextures(n : GLsizei, textures : *GLuint);
    fn glGenTransformFeedbacks(n : GLsizei, ids : *GLuint);
    fn glGenVertexArrays(n : GLsizei, arrays : *GLuint);
    fn glGenerateMipmap(target : GLenum);
    fn glGetBooleanv(pname : GLenum, params : *GLboolean);
    fn glGetDoublev(pname : GLenum, params : *GLdouble);
    fn glGetFloatv(pname : GLenum, params : *GLfloat);
    fn glGetIntegerv(pname : GLenum, params : *GLint);
    fn glGetInteger64v(pname : GLenum, params : *GLint64);
    fn glGetBooleani_v(pname : GLenum, index : GLuint, data : *GLboolean);
    fn glGetIntegeri_v(pname : GLenum, index : GLuint, data : *GLint);
    fn glGetInteger64i_v(pname : GLenum, index : GLuint, data : *GLint64);
    fn glGetActiveAtomicCounterBufferiv(program : GLuint,  bufferIndex : GLuint,  pname : GLenum, params : *GLint);
    fn glGetActiveAttrib(program : GLuint,  index : GLuint,  bufSize : GLsizei, length : *GLsizei, size : *GLint, atype : *GLenum, name : *GLchar);
    fn glGetActiveSubroutineName(program : GLuint, shadertype : GLenum, index : GLuint,  bufSize : GLsizei, length : *GLsizei, name : *GLchar);
    fn glGetActiveSubroutineUniformiv(program : GLuint, shadertype : GLenum,  index : GLuint, pname : GLenum, values : *GLint);
    fn glGetActiveSubroutineUniformName(program : GLuint, shadertype : GLenum,  index : GLuint,  bufSize : GLsizei, length : *GLsizei, name : *GLchar);
    fn glGetActiveUniform(program : GLuint,  index : GLuint,  bufSize : GLsizei, length : *GLsizei, size : *GLint, atype : *GLenum, name : *GLchar);
    fn glGetActiveUniformBlockiv(program : GLuint, uniformBlockIndex : GLuint, pname : GLenum, params : GLint);
    fn glGetActiveUniformBlockName(program : GLuint, uniformBlockIndex : GLuint,  bufSize : GLsizei, length : *GLsizei, uniformBlockName : *GLchar);
    fn glGetActiveUniformName(program : GLuint, uniformIndex : GLuint,  bufSize : GLsizei, length : *GLsizei, uniformName : *GLchar);
    fn glGetActiveUniformsiv(program : GLuint, uniformCount : GLsizei, uniformIndices : *GLuint, pname : GLenum, params : *GLint);
    fn glGetAttachedShaders(program : GLuint, maxCount : GLsizei, count : *GLsizei, shaders : *GLuint);
    fn glGetAttribLocation(program : GLuint, name : *GLchar) -> GLint;
    fn glGetBufferParameteriv(target : GLenum, value : GLenum, data : *GLint);
    fn glGetBufferPointerv(target : GLenum, pname : GLenum, params : **uint);
    fn glGetBufferSubData(target : GLenum, offset : GLintptr, size : GLsizeiptr, data : *uint);
    fn glGetCompressedTexImage(target : GLenum, lod : GLint, img : *uint);
    fn glGetError() -> GLenum;
    fn glGetFragDataIndex(program : GLuint, name : *u8) -> GLint;
    fn glGetFragDataLocation(program : GLuint, name : *u8) -> GLint;
    fn glGetFramebufferAttachmentParameteriv(target : GLenum, attachment : GLenum,  pname : GLenum, params : *GLint); 
    fn glGetInternalformativ(target : GLenum, internalformat : GLenum,  pname : GLenum, bufSize : GLsizei, params : *GLint); 
    fn glGetMultisamplefv(pname : GLenum, index : GLuint, val : *GLfloat);
    fn glGetProgramiv(program : GLuint,  pname : GLenum, params : *GLint);
    fn glGetProgramBinary(program : GLuint,  bufsize : GLsizei,  length : *GLsizei, binaryFormat : *GLenum, binary : *uint);
    fn glGetProgramInfoLog(program : GLuint, maxLength : GLsizei,  length : *GLsizei, infoLog : *GLchar);
    fn glGetProgramPipelineiv(pipeline : GLuint,  pname : GLenum, params : *GLint);
    fn glGetProgramPipelineInfoLog(pipeline : GLuint,  bufsize : GLsizei,  length : *GLsizei, infoLog : *GLchar);
    fn glGetProgramStageiv(program : GLuint,  shadertype : GLenum,  pname : GLenum,  values : *GLint);
    fn glGetQueryIndexediv(target : GLenum,  index : GLuint,  pname : GLenum, params : *GLint);
    fn glGetQueryObjectiv(id : GLuint,  pname : GLenum, params : *GLint);
    fn glGetQueryObjectuiv(id : GLuint,  pname : GLenum,  params : *GLuint);
    fn glGetQueryObjecti64v(id : GLuint,  pname : GLenum,  params : *GLint64);
    fn glGetQueryObjectui64v(id : GLuint,  pname : GLenum,  params : *GLuint64);
    fn glGetQueryiv(target : GLenum,  pname : GLenum,  params : *GLint);
    fn glGetRenderbufferParameteriv(target : GLenum,  pname : GLenum,  params : *GLint);
    fn glGetSamplerParameterfv(sampler : GLuint,  pname : GLenum, params : *GLfloat);
    fn glGetSamplerParameteriv(sampler : GLuint,  pname : GLenum,  params : *GLint);
    fn glGetShaderiv(shader : GLuint,  pname : GLenum, params : *GLint);
    fn glGetShaderInfoLog(shader : GLuint, maxLength : GLsizei,  length : *GLsizei, infoLog : *GLchar);
    fn glGetShaderPrecisionFormat(shadertype : GLenum,  precisionType : GLenum, range : *GLint, precision : *GLint);
    fn glGetShaderSource(shader : GLuint,  bufsize : GLsizei,  length : *GLsizei, source : *GLchar);
    fn glGetString(name : GLenum) -> *GLubyte;
    fn glGetStringi(name : GLenum,  index : GLuint) -> *GLubyte;
    fn glGetSubroutineIndex(program : GLuint,  shadertype : GLenum,  name : *GLchar) -> GLuint;
    fn glGetSubroutineUniformLocation(program : GLuint,  shadertype : GLenum,  name : *GLchar) -> GLint;
    fn glGetSynciv(sync : GLsync,  pname : GLenum,  bufsize : GLsizei,  length : *GLsizei, values : *GLint);
    fn glGetTexImage(target : GLenum,  level : GLint, format : GLenum, ttype : GLenum, img : *uint);
    fn glGetTexLevelParameterfv(target : GLenum,  level : GLint,  pname : GLenum,  params : *GLfloat);
    fn glGetTexLevelParameteriv(target : GLenum,  level : GLint,  pname : GLenum,  params : *GLint);
    fn glGetTexParameterfv(target : GLenum,  pname : GLenum,  params : *GLfloat);
    fn glGetTexParameteriv(target : GLenum,  pname : GLenum,  params : *GLint);
    fn glGetTexParameterIiv(target : GLenum,  pname : GLenum,  params : *GLint);
    fn glGetTexParameterIuiv(target : GLenum,  pname : GLenum,  params : *GLuint);
    fn glGetTransformFeedbackVarying(program : GLuint, index : GLuint, bufSize : GLsizei,  length : *GLsizei, size : GLsizei, ttype : *GLenum,  name : *GLchar);
    fn glGetUniformfv(program : GLuint,  location : GLint,  params : *GLfloat);
    fn glGetUniformiv(program : GLuint,  location : GLint,  params : *GLint);
    fn glGetUniformBlockIndex(program : GLuint,  uniformBlockName : *GLchar) -> GLuint;
    fn glGetUniformIndices(program : GLuint,  uniformCount : GLsizei, uniformNames : **GLchar, uniformIndices : *GLuint) -> GLuint;
    fn glGetUniformLocation(program : GLuint,  name : *GLchar) -> GLint;
    fn glGetUniformSubroutineuiv(shadertype : GLenum,  location : GLint, values : *GLuint);
    fn glGetVertexAttribdv(index : GLuint,  pname : GLenum,  params : *GLdouble);
    fn glGetVertexAttribfv(index : GLuint,  pname : GLenum,  params : *GLfloat);
    fn glGetVertexAttribiv(index : GLuint,  pname : GLenum,  params : *GLint);
    fn glGetVertexAttribIiv(index : GLuint,  pname : GLenum,  params : *GLint);
    fn glGetVertexAttribIuiv(index : GLuint,  pname : GLenum,  params : *GLuint);
    fn glGetVertexAttribLdv(index : GLuint,  pname : GLenum,  params : *GLdouble);
    fn glGetVertexAttribPointerv(index : GLuint,  pname : GLenum,  pointer : **uint);
    
    
    /******** H ********/
    
    fn glHint(target : GLenum, mode : GLenum);
    
    

    /******** I ********/
    
    fn glIsBuffer(buffer : GLuint) -> GLboolean;
    fn glIsEnabled(cap : GLenum) -> GLboolean;
    fn glIsEnabledi(cap : GLenum, index : GLuint) -> GLboolean;
    fn glIsFramebuffer(framebuffer : GLuint) -> GLboolean;
    fn glIsProgram(program : GLuint) -> GLboolean;
    fn glIsProgramPipeline(pipeline : GLuint) -> GLboolean;
    fn glIsQuery(id : GLuint) -> GLboolean;
    fn glIsRenderbuffer(renderbuffer : GLuint) -> GLboolean;
    fn glIsSampler(id : GLuint) -> GLboolean;
    fn glIsShader(shader : GLuint) -> GLboolean;
    fn glIsSync(sync : GLsync) -> GLboolean;
    fn glIsTexture(texture : GLuint) -> GLboolean;
    fn glIsTransformFeedback(id : GLuint) -> GLboolean;
    fn glIsVertexArray(array : GLuint) -> GLboolean;


    /******** L ********/
    
    fn glLineWidth(width : GLfloat);
    fn glLinkProgram(program : GLuint);
    fn glLogicOp(opcode : GLenum);



    /******** M ********/
    
    fn glMapBuffer(target : GLenum, access : GLenum) -> *uint;
    fn glMapBufferRange(target : GLenum, offset : GLintptr, length : GLsizeiptr, access : GLbitfield) -> *uint;
    fn glMemoryBarrier(barriers : GLbitfield);
    fn glMinSampleShading(value : GLclampf);
    fn glMultiDrawArrays(mode : GLenum, first : *GLint, count : *GLsizei, primcount : GLsizei);
    fn glMultiDrawElements(mode : GLenum, count : *GLsizei, etype : GLenum, indices : **uint, primcount : GLsizei);
    fn glMultiDrawElementsBaseVertex(mode : GLenum, count : GLsizei, etype : GLenum, indices : **uint, primcount : GLsizei, basevertex : *GLint);


    /******** P ********/
    
    fn glPatchParameteri(pname : GLenum, value : GLint);
    fn glPatchParameterfv(pname : GLenum, values : *GLfloat);
    fn glPauseTransformFeedback();
    fn glPixelStoref(pname : GLenum, param : GLfloat);
    fn glPixelStorei(pname : GLenum, param : GLint);
    fn glPointParameterf(pname : GLenum, param : GLfloat);
    fn glPointParameteri(pname : GLenum, param : GLint);
    fn glPointSize(size : GLfloat);
    fn glPolygonMode(face : GLenum, mode : GLenum);
    fn glPolygonOffset(factor : GLfloat,  units : GLfloat);
    fn glPrimitiveRestartIndex(index : GLuint);
    fn glProgramBinary(program : GLuint, binaryFormat : GLenum, binary : *uint, length : GLsizei);
    fn glProgramParameteri(program : GLuint, pname : GLenum, value : GLint);
    fn glProgramUniform1f(program : GLuint, location : GLint, v0 : GLfloat);
    fn glProgramUniform2f(program : GLuint, location : GLint, v0 : GLfloat, v1 : GLfloat);
    fn glProgramUniform3f(program : GLuint, location : GLint, v0 : GLfloat, v1 : GLfloat, v2 : GLfloat);
    fn glProgramUniform4f(program : GLuint, location : GLint, v0 : GLfloat, v1 : GLfloat, v2 : GLfloat, v3 : GLfloat);
    fn glProgramUniform1i(program : GLuint, location : GLint, v0 : GLint);
    fn glProgramUniform2i(program : GLuint, location : GLint, v0 : GLint, v1 : GLint);
    fn glProgramUniform3i(program : GLuint, location : GLint, v0 : GLint, v1 : GLint, v2 : GLint);
    fn glProgramUniform4i(program : GLuint, location : GLint, v0 : GLint, v1 : GLint, v2 : GLint, v3 : GLint);
    fn glProgramUniform1ui(program : GLuint, location : GLint, v0 : GLuint);
    fn glProgramUniform2ui(program : GLuint, location : GLint, v0 : GLint, v1 : GLuint);
    fn glProgramUniform3ui(program : GLuint, location : GLint, v0 : GLint, v1 : GLint, v2 : GLuint);
    fn glProgramUniform4ui(program : GLuint, location : GLint, v0 : GLint, v1 : GLint, v2 : GLint, v3 : GLuint);
    fn glProgramUniform1fv(program : GLuint, location : GLint, count : GLsizei, value : *GLfloat);
    fn glProgramUniform2fv(program : GLuint, location : GLint, count : GLsizei, value : *GLfloat);
    fn glProgramUniform3fv(program : GLuint, location : GLint, count : GLsizei, value : *GLfloat);
    fn glProgramUniform4fv(program : GLuint, location : GLint, count : GLsizei, value : *GLfloat);
    fn glProgramUniform1iv(program : GLuint, location : GLint, count : GLsizei, value : *GLint);
    fn glProgramUniform2iv(program : GLuint, location : GLint, count : GLsizei, value : *GLint);
    fn glProgramUniform3iv(program : GLuint, location : GLint, count : GLsizei, value : *GLint);
    fn glProgramUniform4iv(program : GLuint, location : GLint, count : GLsizei, value : *GLint);
    fn glProgramUniform1uiv(program : GLuint, location : GLint, count : GLsizei, value : *GLuint);
    fn glProgramUniform2uiv(program : GLuint, location : GLint, count : GLsizei, value : *GLuint);
    fn glProgramUniform3uiv(program : GLuint, location : GLint, count : GLsizei, value : *GLuint);
    fn glProgramUniform4uiv(program : GLuint, location : GLint, count : GLsizei, value : *GLuint);
    fn glProgramUniformMatrix2fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix3fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix4fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix2x3fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix3x2fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix2x4fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix4x2fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix3x4fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProgramUniformMatrix4x3fv(program : GLuint, location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glProvokingVertex(provokeMode : GLenum);



    /******** Q ********/
    
    fn glQueryCounter(id : GLuint, target : GLenum);



    /******** R ********/
    
    fn glReadBuffer(mode : GLenum);
    fn glReadPixels(x : GLint, y : GLint, width : GLsizei, height : GLsizei, format : GLenum, etype : GLenum, data : *uint);
    fn glReleaseShaderCompiler();
    fn glRenderbufferStorage(target : GLenum, internalformat : GLenum, width : GLsizei, height : GLsizei);
    fn glRenderbufferStorageMultisample(target : GLenum, samples : GLsizei, internalformat : GLenum, width : GLsizei, height : GLsizei);
    fn glResumeTransformFeedback();



    /******** S ********/
    
    fn glSampleCoverage(value : GLclampf, invert : GLboolean);
    fn glSampleMaski(maskNumber : GLuint, mask : GLbitfield);
    fn glSamplerParameterf(sampler : GLuint, pname : GLenum, param : GLfloat);
    fn glSamplerParameteri(sampler : GLuint, pname : GLenum, param : GLint);
    fn glScissor(x : GLint, y : GLint, width : GLsizei, height : GLsizei);
    fn glScissorArrayv(first : GLuint, count : GLsizei, v : *GLint);
    fn glScissorIndexed(index : GLuint, left : GLint, bottom : GLint, width : GLsizei, height : GLsizei);
    fn glScissorIndexedv(index : GLuint, v : *GLint);
    fn glShaderBinary(count : GLsizei, shaders : *GLuint, binaryFormat : GLenum, binary : *uint, length : GLsizei);
    fn glShaderSource(shader : GLuint, count : GLsizei, string : **u8, length : *GLint);
    fn glStencilFunc(func : GLenum, ref : GLint, mask : GLuint);
    fn glStencilFuncSeparate(face : GLenum, func : GLenum, ref : GLint, mask : GLuint);
    fn glStencilMask(mask : GLuint);
    fn glStencilMaskSeparate(face : GLenum, mask : GLuint);
    fn glStencilOp(sfail : GLenum, dpfail : GLenum, dppass : GLenum);
    fn glStencilOpSeparate(face : GLenum, sfail : GLenum, dpfail : GLenum, dppass : GLenum);



    /******** T ********/
    
    fn glTexBuffer(target : GLenum, internalFormat : GLenum, buffer : GLuint);
    fn glTexImage1D(target : GLenum, level : GLint, internalFormat : GLint, width : GLsizei, border : GLint, format : GLenum, ttype : GLenum, data : *uint);
    fn glTexImage2D(target : GLenum, level : GLint, internalFormat : GLint, width : GLsizei, height : GLsizei, border : GLint, format : GLenum, ttype : GLenum, data : *uint);
    fn glTexImage2DMultisample(target : GLenum, samples : GLsizei, internalFormat : GLint, width : GLsizei, height : GLsizei, fixedsamplelocations : GLboolean);
    fn glTexImage3D(target : GLenum, level : GLint, internalFormat : GLint, width : GLsizei, height : GLsizei, depth : GLsizei, border : GLint, format : GLenum, ttype : GLenum, data : *uint);
    fn glTexImage3DMultisample(target : GLenum, samples : GLsizei, internalFormat : GLint, width : GLsizei, height : GLsizei, depth : GLsizei, fixedsamplelocations : GLboolean);
    fn glTexParameterf(target : GLenum, pname : GLenum, param : GLfloat);
    fn glTexParameteri(target : GLenum, pname : GLenum, param : GLint);
    fn glTexParameterfv(target : GLenum, pname : GLenum, params : *GLfloat);
    fn glTexParameteriv(target : GLenum, pname : GLenum, params : *GLint);
    fn glTexParameterIiv(target : GLenum, pname : GLenum, params : *GLint);
    fn glTexParameterIuiv(target : GLenum, pname : GLenum, params : *GLuint);
    fn glTexStorage1D(target : GLenum, levels : GLsizei, internalFormat : GLenum, width : GLsizei);
    fn glTexStorage2D(target : GLenum, levels : GLsizei, internalFormat : GLenum, width : GLsizei, height : GLsizei);
    fn glTexStorage3D(target : GLenum, levels : GLsizei, internalFormat : GLenum, width : GLsizei, height : GLsizei, depth : GLsizei);
    fn glTexSubImage1D(target : GLenum, level : GLint, xoffset : GLint, width : GLsizei, format : GLenum, ttype : GLenum, data : *uint);
    fn glTexSubImage2D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, width : GLsizei, height : GLsizei, format : GLenum, ttype : GLenum, data : *uint);
    fn glTexSubImage3D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, zoffset : GLint, width : GLsizei, height : GLsizei, depth : GLsizei, format : GLenum, ttype : GLenum, data : *uint);
    fn glTransformFeedbackVaryings(program : GLuint, count : GLsizei, varyings : **u8, bufferMode : GLenum);



    /******** U ********/
    
    fn glUniform1f(location : GLint, v0 : GLfloat);
    fn glUniform2f(location : GLint, v0 : GLfloat, v1 : GLfloat);
    fn glUniform3f(location : GLint, v0 : GLfloat, v1 : GLfloat, v2 : GLfloat);
    fn glUniform4f(location : GLint, v0 : GLfloat, v1 : GLfloat, v2 : GLfloat, v3 : GLfloat);
    fn glUniform1i(location : GLint, v0 : GLint);
    fn glUniform2i(location : GLint, v0 : GLint, v1 : GLint);
    fn glUniform3i(location : GLint, v0 : GLint, v1 : GLint, v2 : GLint);
    fn glUniform4i(location : GLint, v0 : GLint, v1 : GLint, v2 : GLint, v3 : GLint);
    fn glUniform1ui(location : GLint, v0 : GLuint);
    fn glUniform2ui(location : GLint, v0 : GLint, v1 : GLuint);
    fn glUniform3ui(location : GLint, v0 : GLint, v1 : GLint, v2 : GLuint);
    fn glUniform4ui(location : GLint, v0 : GLint, v1 : GLint, v2 : GLint, v3 : GLuint);
    fn glUniform1fv(location : GLint, count : GLsizei, value : *GLfloat);
    fn glUniform2fv(location : GLint, count : GLsizei, value : *GLfloat);
    fn glUniform3fv(location : GLint, count : GLsizei, value : *GLfloat);
    fn glUniform4fv(location : GLint, count : GLsizei, value : *GLfloat);
    fn glUniform1iv(location : GLint, count : GLsizei, value : *GLint);
    fn glUniform2iv(location : GLint, count : GLsizei, value : *GLint);
    fn glUniform3iv(location : GLint, count : GLsizei, value : *GLint);
    fn glUniform4iv(location : GLint, count : GLsizei, value : *GLint);
    fn glUniform1uiv(location : GLint, count : GLsizei, value : *GLuint);
    fn glUniform2uiv(location : GLint, count : GLsizei, value : *GLuint);
    fn glUniform3uiv(location : GLint, count : GLsizei, value : *GLuint);
    fn glUniform4uiv(location : GLint, count : GLsizei, value : *GLuint);
    fn glUniformMatrix2fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix3fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix4fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix2x3fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix3x2fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix2x4fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix4x2fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix3x4fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformMatrix4x3fv(location : GLint, count : GLsizei, transpose : GLboolean, value : *GLfloat);
    fn glUniformBlockBinding(program : GLuint, uniformBlockIndex : GLuint, uniformBlockBinding : GLuint);
    fn glUniformSubroutinesuiv(shadertype : GLenum, count : GLsizei, indices : *uint);
    fn glUnmapBuffer(target : GLenum) -> GLboolean;
    fn glUseProgram(program : GLuint);
    fn glUseProgramStages(pipeline : GLuint, stages : GLbitfield, program : GLuint);



    /******** V ********/
    
    fn glValidateProgram(program : GLuint);
    fn glValidateProgramPipeline(pipeline : GLuint);
    fn glVertexAttrib1f(index : GLuint, v0 : GLfloat);
    fn glVertexAttrib1s(index : GLuint, v0 : GLshort);
    fn glVertexAttrib1d(index : GLuint, v0 : GLdouble);
    fn glVertexAttribI1i(index : GLuint, v0 : GLint);
    fn glVertexAttribI1ui(index : GLuint, v0 : GLuint);
    fn glVertexAttrib2f(index : GLuint, v0 : GLfloat, v1 : GLfloat);
    fn glVertexAttrib2s(index : GLuint, v0 : GLshort, v1 : GLshort);
    fn glVertexAttrib2d(index : GLuint, v0 : GLdouble, v1 : GLdouble);
    fn glVertexAttribI2i(index : GLuint, v0 : GLint, v1 : GLint);
    fn glVertexAttribI2ui(index : GLuint, v0 : GLuint, v1 : GLuint);
    fn glVertexAttrib3f(index : GLuint, v0 : GLfloat, v1 : GLfloat, v2 : GLfloat);
    fn glVertexAttrib3s(index : GLuint, v0 : GLshort, v1 : GLshort, v2 : GLshort);
    fn glVertexAttrib3d(index : GLuint, v0 : GLdouble, v1 : GLdouble, v2 : GLdouble);
    fn glVertexAttribI3i(index : GLuint, v0 : GLint, v1 : GLint, v2 : GLint);
    fn glVertexAttribI3ui(index : GLuint, v0 : GLuint, v1 : GLuint, v2 : GLuint);
    fn glVertexAttrib4f(index : GLuint, v0 : GLfloat, v1 : GLfloat, v2 : GLfloat, v3 : GLfloat);
    fn glVertexAttrib4s(index : GLuint, v0 : GLshort, v1 : GLshort, v2 : GLshort, v3 : GLshort);
    fn glVertexAttrib4d(index : GLuint, v0 : GLdouble, v1 : GLdouble, v2 : GLdouble, v3 : GLdouble);
    fn glVertexAttrib4Nub(index : GLuint, v0 : GLubyte, v1 : GLubyte, v2 : GLubyte, v3 : GLubyte);
    fn glVertexAttribI4i(index : GLuint, v0 : GLint, v1 : GLint, v2 : GLint, v3 : GLint);
    fn glVertexAttribI4ui(index : GLuint, v0 : GLuint, v1 : GLuint, v2 : GLuint, v3 : GLuint);
    fn glVertexAttribL1d(index : GLuint, v0 : GLdouble);
    fn glVertexAttribL2d(index : GLuint, v0 : GLdouble, v1 : GLdouble);
    fn glVertexAttribL3d(index : GLuint, v0 : GLdouble, v1 : GLdouble, v2 : GLdouble);
    fn glVertexAttribL4d(index : GLuint, v0 : GLdouble, v1 : GLdouble, v2 : GLdouble, v3 : GLdouble);
    fn glVertexAttrib1fv(index : GLuint, v : *GLfloat);
    fn glVertexAttrib1sv(index : GLuint, v : *GLshort);
    fn glVertexAttrib1dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribI1iv(index : GLuint, v : *GLint);
    fn glVertexAttribI1uiv(index : GLuint, v : *GLuint);
    fn glVertexAttrib2fv(index : GLuint, v : *GLfloat);
    fn glVertexAttrib2sv(index : GLuint, v : *GLshort);
    fn glVertexAttrib2dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribI2iv(index : GLuint, v : *GLint);
    fn glVertexAttribI2uiv(index : GLuint, v : *GLuint);
    fn glVertexAttrib3fv(index : GLuint, v : *GLfloat);
    fn glVertexAttrib3sv(index : GLuint, v : *GLshort);
    fn glVertexAttrib3dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribI3iv(index : GLuint, v : *GLint);
    fn glVertexAttribI3uiv(index : GLuint, v : *GLuint);
    fn glVertexAttrib4fv(index : GLuint, v : *GLfloat);
    fn glVertexAttrib4sv(index : GLuint, v : *GLshort);
    fn glVertexAttrib4dv(index : GLuint, v : *GLdouble);
    fn glVertexAttrib4iv(index : GLuint, v : *GLint);
    fn glVertexAttrib4bv(index : GLuint, v : *GLbyte);
    fn glVertexAttrib4ubv(index : GLuint, v : *GLubyte);
    fn glVertexAttrib4usv(index : GLuint, v : *GLushort);
    fn glVertexAttrib4uiv(index : GLuint, v : *GLuint);
    fn glVertexAttrib4Nbv(index : GLuint, v : *GLbyte);
    fn glVertexAttrib4Nsv(index : GLuint, v : *GLshort);
    fn glVertexAttrib4Niv(index : GLuint, v : *GLint);
    fn glVertexAttrib4Nubv(index : GLuint, v : *GLubyte);
    fn glVertexAttrib4Nusv(index : GLuint, v : *GLushort);
    fn glVertexAttrib4Nuiv(index : GLuint, v : *GLuint);
    fn glVertexAttribI4bv(index : GLuint, v : *GLbyte);
    fn glVertexAttribI4ubv(index : GLuint, v : *GLubyte);
    fn glVertexAttribI4sv(index : GLuint, v : *GLshort);
    fn glVertexAttribI4usv(index : GLuint, v : *GLushort);
    fn glVertexAttribI4iv(index : GLuint, v : *GLint);
    fn glVertexAttribI4uiv(index : GLuint, v : *GLuint);
    fn glVertexAttribL1dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribL2dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribL3dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribL4dv(index : GLuint, v : *GLdouble);
    fn glVertexAttribP1ui(index : GLuint, atype : GLenum, normalized : GLboolean, value : GLuint);
    fn glVertexAttribP2ui(index : GLuint, atype : GLenum, normalized : GLboolean, value : GLuint);
    fn glVertexAttribP3ui(index : GLuint, atype : GLenum, normalized : GLboolean, value : GLuint);
    fn glVertexAttribP4ui(index : GLuint, atype : GLenum, normalized : GLboolean, value : GLuint);
    fn glVertexAttribDivisor(index : GLuint, divisor : GLuint);
    fn glVertexAttribPointer(index : GLuint, size : GLint, atype : GLenum, normalized : GLboolean, stride : GLsizei, pointer : *uint);
    fn glVertexAttribIPointer(index : GLuint, size : GLint, atype : GLenum, stride : GLsizei, pointer : *uint);
    fn glVertexAttribLPointer(index : GLuint, size : GLint, atype : GLenum, stride : GLsizei, pointer : *uint);
    fn glViewport(x : GLint, y : GLint, width : GLsizei, height : GLsizei);
    fn glViewportArrayv(first : GLuint, count : GLsizei, v : *GLfloat);
    fn glViewportIndexedf(index : GLuint, x : GLfloat, y : GLfloat, w : GLfloat, h : GLfloat);
    fn glViewportIndexedfv(index : GLuint, v : *GLfloat);


    /******** W ********/

    fn glWaitSync(sync : GLsync, flags : GLbitfield, timeout : GLuint64);    
    
}

