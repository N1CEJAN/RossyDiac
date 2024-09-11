// Copyright 2016 Proyectos y Sistemas de Mantenimiento SL (eProsima).
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*!
 * @file Ros2Arrayspezifikationen.cpp
 * This source file contains the definition of the described types in the IDL file.
 *
 * This file was generated by the tool gen.
 */

#ifdef _WIN32
// Remove linker warning LNK4221 on Visual Studio
namespace {
char dummy;
}  // namespace
#endif  // _WIN32

#include "Ros2Arrayspezifikationen.h"
#include <fastcdr/Cdr.h>

#include <fastcdr/exceptions/BadParamException.h>
using namespace eprosima::fastcdr::exception;

#include <utility>


#define conversion_tests_msg_Ros2Arrayspezifikationen_max_cdr_typesize 127ULL;

#define conversion_tests_msg_Ros2Arrayspezifikationen_max_key_cdr_typesize 0ULL;


conversion_tests::msg::Ros2Arrayspezifikationen::Ros2Arrayspezifikationen()
{
    // conversion_tests::msg::boolean__2 m_static_array
    memset(&m_static_array, 0, (2) * 1);
    // sequence<boolean> m_dynamic_array

    // sequence<string, 2> m_bound_dynamic_array


}

conversion_tests::msg::Ros2Arrayspezifikationen::~Ros2Arrayspezifikationen()
{



}

conversion_tests::msg::Ros2Arrayspezifikationen::Ros2Arrayspezifikationen(
        const Ros2Arrayspezifikationen& x)
{
    m_static_array = x.m_static_array;
    m_dynamic_array = x.m_dynamic_array;
    m_bound_dynamic_array = x.m_bound_dynamic_array;
}

conversion_tests::msg::Ros2Arrayspezifikationen::Ros2Arrayspezifikationen(
        Ros2Arrayspezifikationen&& x) noexcept 
{
    m_static_array = std::move(x.m_static_array);
    m_dynamic_array = std::move(x.m_dynamic_array);
    m_bound_dynamic_array = std::move(x.m_bound_dynamic_array);
}

conversion_tests::msg::Ros2Arrayspezifikationen& conversion_tests::msg::Ros2Arrayspezifikationen::operator =(
        const Ros2Arrayspezifikationen& x)
{

    m_static_array = x.m_static_array;
    m_dynamic_array = x.m_dynamic_array;
    m_bound_dynamic_array = x.m_bound_dynamic_array;

    return *this;
}

conversion_tests::msg::Ros2Arrayspezifikationen& conversion_tests::msg::Ros2Arrayspezifikationen::operator =(
        Ros2Arrayspezifikationen&& x) noexcept
{

    m_static_array = std::move(x.m_static_array);
    m_dynamic_array = std::move(x.m_dynamic_array);
    m_bound_dynamic_array = std::move(x.m_bound_dynamic_array);

    return *this;
}

bool conversion_tests::msg::Ros2Arrayspezifikationen::operator ==(
        const Ros2Arrayspezifikationen& x) const
{

    return (m_static_array == x.m_static_array && m_dynamic_array == x.m_dynamic_array && m_bound_dynamic_array == x.m_bound_dynamic_array);
}

bool conversion_tests::msg::Ros2Arrayspezifikationen::operator !=(
        const Ros2Arrayspezifikationen& x) const
{
    return !(*this == x);
}

size_t conversion_tests::msg::Ros2Arrayspezifikationen::getMaxCdrSerializedSize(
        size_t current_alignment)
{
    static_cast<void>(current_alignment);
    return conversion_tests_msg_Ros2Arrayspezifikationen_max_cdr_typesize;
}

size_t conversion_tests::msg::Ros2Arrayspezifikationen::getCdrSerializedSize(
        const conversion_tests::msg::Ros2Arrayspezifikationen& data,
        size_t current_alignment)
{
    (void)data;
    size_t initial_alignment = current_alignment;


    current_alignment += ((2) * 1) + eprosima::fastcdr::Cdr::alignment(current_alignment, 1);

    current_alignment += 4 + eprosima::fastcdr::Cdr::alignment(current_alignment, 4);

    if (data.dynamic_array().size() > 0)
    {
        current_alignment += (data.dynamic_array().size() * 1) + eprosima::fastcdr::Cdr::alignment(current_alignment, 1);
    }



    current_alignment += 4 + eprosima::fastcdr::Cdr::alignment(current_alignment, 4);


    for(size_t a = 0; a < data.bound_dynamic_array().size(); ++a)
    {
        current_alignment += 4 + eprosima::fastcdr::Cdr::alignment(current_alignment, 4) +
            data.bound_dynamic_array().at(a).size() + 1;
    }

    return current_alignment - initial_alignment;
}

void conversion_tests::msg::Ros2Arrayspezifikationen::serialize(
        eprosima::fastcdr::Cdr& scdr) const
{

    scdr << m_static_array;

    scdr << m_dynamic_array;
    {
        scdr << static_cast<uint32_t>(m_bound_dynamic_array.size());
        for (const auto& item : m_bound_dynamic_array)
        {
            scdr << item.c_str();
        }
    }

}

void conversion_tests::msg::Ros2Arrayspezifikationen::deserialize(
        eprosima::fastcdr::Cdr& dcdr)
{

    dcdr >> m_static_array;

    dcdr >> m_dynamic_array;
    {
        uint32_t sequence_size = 0;
        dcdr >> sequence_size;
        m_bound_dynamic_array.resize(sequence_size);
        for (auto& item : m_bound_dynamic_array)
        {
            std::string s;
            dcdr >> s;
            item = s.c_str();
        }
    }

}

/*!
 * @brief This function copies the value in member static_array
 * @param _static_array New value to be copied in member static_array
 */
void conversion_tests::msg::Ros2Arrayspezifikationen::static_array(
        const conversion_tests::msg::boolean__2& _static_array)
{
    m_static_array = _static_array;
}

/*!
 * @brief This function moves the value in member static_array
 * @param _static_array New value to be moved in member static_array
 */
void conversion_tests::msg::Ros2Arrayspezifikationen::static_array(
        conversion_tests::msg::boolean__2&& _static_array)
{
    m_static_array = std::move(_static_array);
}

/*!
 * @brief This function returns a constant reference to member static_array
 * @return Constant reference to member static_array
 */
const conversion_tests::msg::boolean__2& conversion_tests::msg::Ros2Arrayspezifikationen::static_array() const
{
    return m_static_array;
}

/*!
 * @brief This function returns a reference to member static_array
 * @return Reference to member static_array
 */
conversion_tests::msg::boolean__2& conversion_tests::msg::Ros2Arrayspezifikationen::static_array()
{
    return m_static_array;
}
/*!
 * @brief This function copies the value in member dynamic_array
 * @param _dynamic_array New value to be copied in member dynamic_array
 */
void conversion_tests::msg::Ros2Arrayspezifikationen::dynamic_array(
        const std::vector<bool>& _dynamic_array)
{
    m_dynamic_array = _dynamic_array;
}

/*!
 * @brief This function moves the value in member dynamic_array
 * @param _dynamic_array New value to be moved in member dynamic_array
 */
void conversion_tests::msg::Ros2Arrayspezifikationen::dynamic_array(
        std::vector<bool>&& _dynamic_array)
{
    m_dynamic_array = std::move(_dynamic_array);
}

/*!
 * @brief This function returns a constant reference to member dynamic_array
 * @return Constant reference to member dynamic_array
 */
const std::vector<bool>& conversion_tests::msg::Ros2Arrayspezifikationen::dynamic_array() const
{
    return m_dynamic_array;
}

/*!
 * @brief This function returns a reference to member dynamic_array
 * @return Reference to member dynamic_array
 */
std::vector<bool>& conversion_tests::msg::Ros2Arrayspezifikationen::dynamic_array()
{
    return m_dynamic_array;
}
/*!
 * @brief This function copies the value in member bound_dynamic_array
 * @param _bound_dynamic_array New value to be copied in member bound_dynamic_array
 */
void conversion_tests::msg::Ros2Arrayspezifikationen::bound_dynamic_array(
        const std::vector<eprosima::fastrtps::fixed_string<2>>& _bound_dynamic_array)
{
    m_bound_dynamic_array = _bound_dynamic_array;
}

/*!
 * @brief This function moves the value in member bound_dynamic_array
 * @param _bound_dynamic_array New value to be moved in member bound_dynamic_array
 */
void conversion_tests::msg::Ros2Arrayspezifikationen::bound_dynamic_array(
        std::vector<eprosima::fastrtps::fixed_string<2>>&& _bound_dynamic_array)
{
    m_bound_dynamic_array = std::move(_bound_dynamic_array);
}

/*!
 * @brief This function returns a constant reference to member bound_dynamic_array
 * @return Constant reference to member bound_dynamic_array
 */
const std::vector<eprosima::fastrtps::fixed_string<2>>& conversion_tests::msg::Ros2Arrayspezifikationen::bound_dynamic_array() const
{
    return m_bound_dynamic_array;
}

/*!
 * @brief This function returns a reference to member bound_dynamic_array
 * @return Reference to member bound_dynamic_array
 */
std::vector<eprosima::fastrtps::fixed_string<2>>& conversion_tests::msg::Ros2Arrayspezifikationen::bound_dynamic_array()
{
    return m_bound_dynamic_array;
}


size_t conversion_tests::msg::Ros2Arrayspezifikationen::getKeyMaxCdrSerializedSize(
        size_t current_alignment)
{
    static_cast<void>(current_alignment);
    return conversion_tests_msg_Ros2Arrayspezifikationen_max_key_cdr_typesize;
}

bool conversion_tests::msg::Ros2Arrayspezifikationen::isKeyDefined()
{
    return false;
}

void conversion_tests::msg::Ros2Arrayspezifikationen::serializeKey(
        eprosima::fastcdr::Cdr& scdr) const
{
    (void) scdr;
}



