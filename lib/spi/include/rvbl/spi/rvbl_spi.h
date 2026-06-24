/// = SPI Device Library
/// Quintauris GmbH
///
/// Provides generic interfaces to SPI devices.

#ifndef RVBL_SPI_H
#define RVBL_SPI_H

/// == <rvbl_spi.h>

#include "rvbl/type/rvbl_types.h"

/// === Enumeration `rvbl_spi_mode`
///
/// SPI operating mode.
typedef enum rvbl_spi_mode
{
    /// Controller drives the clock and initiates transfers.
    rvbl_spi_mode_master = 0,
    /// Peripheral responds to a controller-initiated transfer.
    rvbl_spi_mode_slave = 1,
    /// Peripheral works in loopback mode.
    rvbl_spi_mode_loopback = 2
} rvbl_spi_mode;

/// === Enumeration `rvbl_spi_clock_polarity`
///
/// Idle state of the SPI clock line (CPOL).
typedef enum rvbl_spi_clock_polarity
{
    /// Clock idles low (CPOL = 0).
    rvbl_spi_clock_polarity_low = 0,
    /// Clock idles high (CPOL = 1).
    rvbl_spi_clock_polarity_high = 1
} rvbl_spi_clock_polarity;

/// === Type `rvbl_spi_clock_phase`
///
/// Clock edge on which data is sampled (CPHA).
typedef enum rvbl_spi_clock_phase
{
    /// Data sampled on the first clock edge (CPHA = 0).
    rvbl_spi_clock_phase_first = 0,
    /// Data sampled on the second clock edge (CPHA = 1).
    rvbl_spi_clock_phase_second = 1
} rvbl_spi_clock_phase;

/// === Type `rvbl_spi_configuration`
///
/// Set of SPI bus parameters.
typedef struct rvbl_spi_configuration
{
    /// `rvbl_spi_mode mode`:: Operating mode (master or slave).
    rvbl_spi_mode mode;
    /// `rvbl_spi_clock_polarity clock_polarity`:: Clock polarity.
    rvbl_spi_clock_polarity clock_polarity;
    /// `rvbl_spi_clock_phase clock_phase`:: Clock phase.
    rvbl_spi_clock_phase clock_phase;
    /// `rvbl_uint32_t frequency`:: SPI CLK frequency, in Hz.
    rvbl_uint32_t frequency;
    /// `rvbl_uint32_t word_size`:: SPI word size, in bits.
    rvbl_uint32_t word_size;
    /// `rvbl_pointer_t vendor`:: Vendor-specific configuration/data.
    rvbl_pointer_t vendor;
} rvbl_spi_configuration;

/// === Type `rvbl_spi_init`
///
/// Function pointer to an SPI device initialization function.
///
/// ==== Parameters
/// `const void*`:: Pointer to device instance.
/// `const rvbl_spi_configuration*`:: Pointer to SPI configuration.
///
/// ==== Return Value
/// `rvbl_result_t`:: `rvbl_result_success` if initialization has been successful.
typedef rvbl_result_t (*rvbl_spi_init)(const void *, const rvbl_spi_configuration *);

/// === Type `rvbl_spi_transcieve`
///
/// Function pointer to an SPI device transmit/receive function. The
/// implementation of this interface behaves differently according to the bus
/// mode the peripheral is operating on:
///
/// master:: Words in transmit buffer are transmitted, then the function blocks
/// until the same number of words have been retrieved from the peripheral
/// into the receive buffer.
/// slave:: Words in transmit buffer are loaded into the peripheral for
/// transmission when bytes are received, then the function blocks until the
/// same number of words have been retrieved from the peripheral into the
/// receive buffer.
///
/// ==== Parameters
/// `const void*`:: Pointer to device instance.
/// `const void*`:: Pointer to data buffer to transmit.
/// `void*`:: Pointer to buffer that receives incoming data.
/// `rvbl_uword_t*`:: (in) Number of words available transmit/receive buffers,
/// (out) number of words received.
///
/// ==== Return Value
/// `rvbl_result_t`:: `rvbl_result_success` if transaction has been successful.
typedef rvbl_result_t (*rvbl_spi_transcieve)(const void *, const void *, void *, rvbl_uword_t *);

/// === Type `rvbl_spi_busy`
///
/// Function pointer to an SPI busy status function.
///
/// ==== Parameters
/// `const void*`:: Pointer to device instance.
///
/// ==== Return Value
/// `rvbl_bool_t`:: Whether SPI device is busy on a transaction.
typedef rvbl_bool_t (*rvbl_spi_busy)(const void *);

/// === Type `rvbl_spi`
///
/// Set of function pointers implementing a particular SPI device.
typedef struct rvbl_spi
{
    /// `rvbl_spi_init init`:: Initialization function.
    rvbl_spi_init init;

    /// `rvbl_spi_transcieve transcieve`:: Transmit function.
    rvbl_spi_transcieve transcieve;

    /// `rvbl_spi_busy busy`:: Busy function.
    rvbl_spi_busy busy;
} rvbl_spi;

#endif
